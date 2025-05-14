use crate::db::types::{ClickHouseDatabase, TableRecord};
use crate::global::{get_ck_db, get_kv};
use anyhow::Result;
use clickhouse::Row;
use serde::Serialize;
use serde::de::DeserializeOwned;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::Mutex;
use tracing::debug;

const KLINE_BATCH_SIZE: usize = 1000;
const MAX_CACHE_DURATION: Duration = Duration::from_secs(10);
pub const SEGMENT_RECENT: &str = "recent";
pub const SEGMENT_HISTORY: &str = "history";

/// 刷新控制器（持有计时器状态，支持并发访问）
pub struct FlushController {
    last_flush: Arc<Mutex<Instant>>, // 使用 Arc 和 Mutex 包装 last_flush，以支持并发访问
    batch_size: usize,
    max_duration: Duration,
}

impl FlushController {
    /// 创建一个新的刷新控制器
    pub fn new(batch_size: usize, max_duration: Duration) -> Self {
        Self {
            last_flush: Arc::new(Mutex::new(Instant::now())),
            batch_size,
            max_duration,
        }
    }

    /// 判断是否需要刷新，并执行刷新函数
    fn make_kline_key(
        &self,
        exchange: &str,
        symbol: &str,
        interval: &str,
        segment: &str,
    ) -> String {
        format!("kline:{}:{}:{}:{}", exchange, symbol, interval, segment)
    }

    pub async fn push<T: Serialize>(
        &self,
        exchange: &str,
        symbol: &str,
        interval: &str,
        segment: &str,
        kline: &T,
    ) -> Result<()> {
        let key = self.make_kline_key(exchange, symbol, interval, segment);
        get_kv().push_kline(&key, kline).await
    }

    pub async fn flush_if_needed<T>(&self) -> Result<()>
    where
        T: for<'de> DeserializeOwned + TableRecord + Row + Serialize,
    {
        // 拉取所有 recent 类型的 Kline（通配所有 symbol, exchange, interval, segment）
        let data: Vec<T> = get_kv().pop_all_by_pattern("kline:*:*:*:*").await?;
        // 获取所有 archive 类型的 Redis 列表总元素数量
        let len = get_kv().len_by_pattern("kline:*:*:*:*").await?;

        let last_flush_guard = self.last_flush.lock().await;

        if len >= self.batch_size || last_flush_guard.elapsed() >= self.max_duration {
            if !data.is_empty() {
                get_ck_db().insert_batch(&data).await?;
                *self.last_flush.lock().await = Instant::now(); // 更新 last_flush
                debug!("Flushed {} kline rows to storage", len);
            }
        }

        Ok(())
    }
}
