use crate::model::cex::kline::{MarketKline, MinMaxCloseTime};
use crate::model::dex::price::PriceUpdate;
use anyhow::{Context, Result};
use clickhouse::inserter::Inserter;
use clickhouse::{Client, Row};
use futures::future::join_all;
use serde::Serialize;
use std::collections::HashMap;
use std::{sync::Arc, time::Duration};
use tokio::sync::RwLock;
use tracing::{debug, info};

pub trait TableRecord: Row + Sized + Send + Sync + Clone + 'static {
    const TABLE_NAME: &'static str;
    fn to_enum_inserter<'a>(inserter: &'a AnyInserter) -> Option<&'a RwLock<Inserter<Self>>>;
}

#[async_trait::async_trait]
pub trait Database {
    fn new(database_url: &str, password: &str, user: &str, database: &str) -> Self
    where
        Self: Sized;
    async fn initialize(&mut self) -> Result<()>;

    async fn health_check(&self) -> Result<()>;

    async fn insert<T>(&self, data: &T, commit: bool) -> Result<()>
    where
        T: TableRecord + Row + Serialize;
    async fn insert_batch<T>(&self, data: &[T]) -> Result<()>
    where
        T: TableRecord + Row + Serialize;

    async fn create_table_if_not_exists(&self, table_name: &str, create_query: &str) -> Result<()>;
}

pub enum AnyInserter {
    PriceUpdate(Arc<RwLock<Inserter<PriceUpdate>>>),
    MarketKline(Arc<RwLock<Inserter<MarketKline>>>),
    // 其他表类型可继续添加
}

pub struct ClickhouseDb {
    client: Client,
    inserters: HashMap<String, AnyInserter>,
    is_initialized: bool,
    max_rows: u64,
}

impl ClickhouseDb {
    fn create_inserter<T: TableRecord>(&self) -> Result<Inserter<T>> {
        Ok(self
            .client
            .inserter::<T>(T::TABLE_NAME)
            .context(format!("failed to prepare insert for {}", T::TABLE_NAME))?
            .with_timeouts(Some(Duration::from_secs(5)), Some(Duration::from_secs(20)))
            .with_max_rows(self.max_rows)
            .with_max_bytes(1_000_000)
            .with_period(Some(Duration::from_secs(15))))
    }
}

#[async_trait::async_trait]
impl Database for ClickhouseDb {
    fn new(database_url: &str, password: &str, user: &str, database: &str) -> Self {
        let client = Client::default()
            .with_url(database_url)
            .with_password(password)
            .with_user(user)
            .with_database(database);

        info!("Connecting to ClickHouse at {}", database_url);
        Self {
            client,
            inserters: HashMap::new(),
            is_initialized: false,
            max_rows: 1000,
        }
    }

    async fn health_check(&self) -> Result<()> {
        debug!("clickhouse healthz");
        self.client
            .query("SELECT 1")
            .execute()
            .await
            .context("Failed to execute health check query")?;
        Ok(())
    }

    async fn initialize(&mut self) -> Result<()> {
        debug!("initializing clickhouse");

        // Define the table creation queries
        let tables = vec![
            // Price updates table
            (
                "price_updates",
                r#"
            CREATE TABLE IF NOT EXISTS price_updates (
                name String,
                pubkey String,
                price Float64,
                market_cap Float64,
                timestamp UInt64,
                slot UInt64,
                swap_amount Float64,
                owner String,
                signature String,
                multi_hop Bool,
                is_buy Bool,
                is_pump Bool,
                INDEX idx_mints (name, pubkey) TYPE minmax GRANULARITY 1
            ) ENGINE = MergeTree()
            ORDER BY (name, pubkey, timestamp)
        "#,
            ),
            // Market Klines table
            (
                "market_klines",
                r#"
            CREATE TABLE IF NOT EXISTS market_klines (
                exchange String,
                symbol String,
                period String,
                open_time UInt64,
                open Float64,
                high Float64,
                low Float64,
                close Float64,
                volume Float64,
                close_time UInt64,
                quote_asset_volume Float64,
                number_of_trades UInt64,
                taker_buy_base_asset_volume Float64,
                taker_buy_quote_asset_volume Float64,
                updated_at DateTime DEFAULT now(),
                PRIMARY KEY (exchange, symbol, period, close_time)
            ) ENGINE = ReplacingMergeTree(updated_at)
            ORDER BY (exchange, symbol, period, close_time)
        "#,
            ),
        ];

        // Execute the creation queries concurrently
        let creation_futures: Vec<_> = tables
            .into_iter()
            .map(|(table_name, query)| self.create_table_if_not_exists(table_name, query))
            .collect();

        // Run all the table creation queries concurrently
        join_all(creation_futures)
            .await
            .into_iter()
            .collect::<Result<()>>()?;

        // Initialize inserter and set initialized flag

        let price_ins = Arc::new(RwLock::new(self.create_inserter::<PriceUpdate>()?));
        let kline_ins = Arc::new(RwLock::new(self.create_inserter::<MarketKline>()?));

        self.inserters.insert(
            "price_updates".to_string(),
            AnyInserter::PriceUpdate(price_ins),
        );
        self.inserters.insert(
            "market_klines".to_string(),
            AnyInserter::MarketKline(kline_ins),
        );

        self.is_initialized = true;

        Ok(())
    }

    async fn create_table_if_not_exists(&self, table_name: &str, create_query: &str) -> Result<()> {
        self.client
            .query(create_query)
            .execute()
            .await
            .context(format!("Failed to create table: {}", table_name))?;
        Ok(())
    }

    /// insert_price uses a batched writer to avoid spamming writes
    async fn insert<T>(&self, data: &T, commit: bool) -> Result<()>
    where
        T: TableRecord + Row + Serialize,
    {
        let _res = match self.inserters.get(T::TABLE_NAME) {
            Some(inserter) => {
                if let Some(typed_inserter) = T::to_enum_inserter(inserter) {
                    let mut inserter = typed_inserter.write().await;

                    inserter
                        .write(data)
                        .context("Failed to write price to insert buffer")?;

                    let pending = inserter.pending();

                    if commit {
                        let stats = inserter.commit().await?;
                        debug!("Committed {} rows ({} bytes)", stats.rows, stats.bytes);
                    } else {
                        if pending.rows >= self.max_rows {
                            let stats = inserter.commit().await?;
                            debug!("Committed {} rows ({} bytes)", stats.rows, stats.bytes);
                        }
                    }

                    Ok(())
                } else {
                    Err(anyhow::anyhow!("Type mismatch for {}", T::TABLE_NAME))
                }
            }
            None => Err(anyhow::anyhow!("No inserter found for {}", T::TABLE_NAME)),
        }
        .expect("inserter error");

        Ok(())
    }

    async fn insert_batch<T>(&self, data: &[T]) -> Result<()>
    where
        T: TableRecord + Row + Serialize,
    {
        if data.is_empty() {
            return Ok(());
        }

        let _res = match self.inserters.get(T::TABLE_NAME) {
            Some(inserter) => {
                if let Some(typed_inserter) = T::to_enum_inserter(inserter) {
                    let mut lock = typed_inserter.write().await;
                    for item in data {
                        lock.write(item).context("Batch insert failed")?;
                    }
                    let stats = lock.commit().await?;
                    debug!("Committed {} rows ({} bytes)", stats.rows, stats.bytes);
                    Ok(())
                } else {
                    Err(anyhow::anyhow!("Type mismatch for {}", T::TABLE_NAME))
                }
            }
            None => Err(anyhow::anyhow!("No inserter found for {}", T::TABLE_NAME)),
        }
        .expect("inserter error");

        Ok(())
    }
}

impl ClickhouseDb {
    /// 查询指定交易所、币对、周期的最早和最晚时间
    pub async fn get_mima_time(
        &self,
        exchange: &str,
        symbol: &str,
        period: &str,
    ) -> Result<Option<MinMaxCloseTime>> {
        let query = r#"
            SELECT
                min(close_time) AS min_close_time,
                max(close_time) AS max_close_time
            FROM market_klines
            WHERE exchange = ? AND symbol = ? AND period = ?
        "#;

        let mut rows = self
            .client
            .query(query)
            .bind(exchange)
            .bind(symbol)
            .bind(period)
            .fetch_all::<MinMaxCloseTime>()
            .await
            .context("Failed to fetch min/max close_time")?;

        // 可能返回一行，但字段为空（表中无匹配记录），可根据是否为0进行判断
        if let Some(result) = rows.pop() {
            if result.min_close_time == 0 && result.max_close_time == 0 {
                Ok(None)
            } else {
                Ok(Some(result))
            }
        } else {
            Ok(None)
        }
    }
    /// 查询指定交易所、币对、周期、时间范围内的k线数据
    /// 时间范围可选，默认查询最近1000条数据
    pub async fn query_market_klines(
        &self,
        exchange: &str,
        symbol: &str,
        period: &str,
        start_time: Option<u64>,
        end_time: Option<u64>,
        limit: Option<u64>,
    ) -> Result<Vec<MarketKline>> {
        let mut sql = String::from(
            r#"
        SELECT
            exchange,
            symbol,
            period,
            open_time,
            open,
            high,
            low,
            close,
            volume,
            close_time,
            quote_asset_volume,
            number_of_trades,
            taker_buy_base_asset_volume,
            taker_buy_quote_asset_volume,
            updated_at
        FROM market_klines
        WHERE exchange = ? AND symbol = ? AND period = ?
    "#,
        );

        // 构造查询
        let mut query = self
            .client
            .query(&sql)
            .bind(exchange)
            .bind(symbol)
            .bind(period);

        // 附加时间过滤
        if let (Some(start), Some(end)) = (start_time, end_time) {
            sql.push_str(" AND close_time BETWEEN ? AND ?");
            query = self
                .client
                .query(&sql)
                .bind(exchange)
                .bind(symbol)
                .bind(period)
                .bind(start)
                .bind(end);
        }

        // ORDER BY + LIMIT
        sql.push_str(" ORDER BY close_time DESC LIMIT ?");
        query = self
            .client
            .query(&sql)
            .bind(exchange)
            .bind(symbol)
            .bind(period);

        if let (Some(start), Some(end)) = (start_time, end_time) {
            query = query.bind(start).bind(end);
        }

        query = query.bind(limit.unwrap_or(1000));

        // 执行查询
        query
            .fetch_all::<MarketKline>()
            .await
            .with_context(|| {
                format!(
                    "Failed to query market_klines: exchange={}, symbol={}, period={}, start={:?}, end={:?}",
                    exchange, symbol, period, start_time, end_time
                )
            })
    }
}

#[cfg(test)]
mod tests {
    use crate::util::make_db;

    use super::*;

    #[tokio::test]
    async fn test_health_check() {
        let db = make_db().await.unwrap();
        db.health_check().await.unwrap();
    }
}
