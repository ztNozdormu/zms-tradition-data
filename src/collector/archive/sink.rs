use crate::collector::archive::{IntoSinkRows, KlineMessage};
use crate::domain::model::market_kline::NewOrUpdateMarketKline as MysqlKline;
use crate::domain::repository::market_kline_repository::MarketKlineRepository;
use crate::domain::service::market_kline_service::MarketKlineService;
use crate::global::{get_ck_db, get_mysql_pool};
use crate::infra::db::types::ClickHouseDatabase;
use crate::model::cex::kline::MarketKline as ClickhouseKline;
use anyhow::Ok;
use async_trait::async_trait;

#[async_trait]
pub trait KlineSink: Send + Sync {
    async fn write(&self, data: Vec<KlineMessage>) -> Result<(), anyhow::Error>;
}

pub struct MysqlSink;

#[async_trait]
impl KlineSink for MysqlSink {
    async fn write(&self, data: Vec<KlineMessage>) -> Result<(), anyhow::Error> {
        let items: Vec<MysqlKline> = data.iter().flat_map(|m| m.into_sink_rows()).collect();

        let mut conn = get_mysql_pool().get()?;
        let repo = MarketKlineRepository::new(&mut conn);
        let mut market_kline_service = MarketKlineService { repo };

        Ok(market_kline_service.save_coin_rank_info(items).await?)
    }
}

pub struct ClickhouseSink;

#[async_trait]
impl KlineSink for ClickhouseSink {
    async fn write(&self, data: Vec<KlineMessage>) -> Result<(), anyhow::Error> {
        let items: Vec<ClickhouseKline> = data.iter().flat_map(|m| m.into_sink_rows()).collect();

        get_ck_db().insert_batch(&items).await
    }
}
