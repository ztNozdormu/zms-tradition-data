use anyhow::Result;
use backoff::{future::retry, ExponentialBackoff};
/// This file contains the implementation of the maintenance module of the trade consumer.
/// 对历史数据进行清理、归档、缓存等操作
use barter::barter_xchange::exchange::binance::api::Binance;
use barter::barter_xchange::exchange::binance::futures::market::FuturesMarket;
use barter::barter_xchange::exchange::binance::model::KlineSummary;
use chrono::Utc;
use futures_util::TryFutureExt;
use tracing::{info, warn};
use crate::db::ckdb::Database;
use crate::global::get_ck_db;
use crate::model::TimeFrame;


// ========== Input Structures ==========

#[derive(Debug, Clone)]
pub struct ArchiveTask {
    pub symbol: String,
    pub exchange: String,
    pub tf: TimeFrame,
    pub window: ArchiveWindow,
    pub direction: ArchiveDirection,
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub enum ArchiveDirection {
    Forward,
    Backward,
}

#[derive(Debug, Clone)]
pub struct ArchiveWindow {
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
}

// ========== Archive Progress Table Logic ==========

#[derive(Debug, Clone)]
pub struct ArchiveProgress {
    pub symbol: String,
    pub exchange: String,
    pub tf: String,
    pub direction: String,
    pub last_processed_time: i64,
    pub completed: bool,
}

pub struct ProgressTracker;

impl ProgressTracker {
    pub async fn get_progress(
        symbol: &str,
        exchange: &str,
        tf: &str,
        direction: &str,
    ) -> Option<ArchiveProgress> {
        // Load from ClickHouse or other DB
        todo!()
    }

    pub async fn update_progress(progress: ArchiveProgress) {
        // Upsert into DB
        todo!()
    }
}

// ========== Main Archive Logic ==========

pub async fn run_archive_task(task: ArchiveTask) -> Result<()> {
    let tf_str = task.tf.to_str();
    let tf_ms = task.tf.to_period();

    let mut current_time = match task.window.start_time {
        Some(start) => start,
        None => ProgressTracker::get_progress(
            &task.symbol,
            &task.exchange,
            tf_str,
            match task.direction {
                ArchiveDirection::Forward => "forward",
                ArchiveDirection::Backward => "backward",
            },
        ).await.map(|p| p.last_processed_time + tf_ms).unwrap_or_else(|| {
            match task.direction {
                ArchiveDirection::Forward => Utc::now().timestamp_millis() - 86_400_000,
                ArchiveDirection::Backward => Utc::now().timestamp_millis(),
            }
        }),
    };

    let end_time = task.window.end_time.unwrap_or_else(|| Utc::now().timestamp_millis());

    let fetcher = BinanceFetcher::new();
    let writer = ClickhouseWriter::new();

    while match task.direction {
        ArchiveDirection::Forward => current_time < end_time,
        ArchiveDirection::Backward => current_time > end_time,
    } {
        let range_start = current_time;
        let range_end = match task.direction {
            ArchiveDirection::Forward => current_time + tf_ms * 1000,
            ArchiveDirection::Backward => current_time - tf_ms * 1000,
        };

        let (start, end) = if task.direction == ArchiveDirection::Forward {
            (range_start, range_end)
        } else {
            (range_end, range_start)
        };

        let klines = retry(ExponentialBackoff::default(), || async {
            fetcher.klines(&task.symbol, tf_str, Some(1000), Some(start as u64), Some(end as u64)).await
                .map_err(|e| {
                    warn!(?e, "Failed to fetch Klines, retrying");
                    backoff::Error::transient(e)
                })
        }).await?;

        if klines.is_empty() {
            info!("No data between {start} ~ {end}");
            break;
        }

        // Gap detection here (optional)
        if !is_kline_continuous(&klines, tf_ms) {
            warn!("Gap detected in klines between {start} ~ {end}");
            // You may handle/fill gaps here
        }

        // writer.write_batch(&klines).await;

        // Update progress
        let last_ts = klines.iter().map(|k| k.close_time).max().unwrap_or(current_time);
        ProgressTracker::update_progress(ArchiveProgress {
            symbol: task.symbol.clone(),
            exchange: task.exchange.clone(),
            tf: tf_str.to_string(),
            direction: match task.direction {
                ArchiveDirection::Forward => "forward".into(),
                ArchiveDirection::Backward => "backward".into(),
            },
            last_processed_time: last_ts,
            completed: last_ts >= end_time,
        }).await;

        current_time = match task.direction {
            ArchiveDirection::Forward => last_ts + tf_ms,
            ArchiveDirection::Backward => last_ts - tf_ms,
        };
    }
    Ok(())
}

// ========== Helper & Placeholder Stubs ==========

fn is_kline_continuous(klines: &[KlineSummary], tf_ms: i64) -> bool {
    klines.windows(2).all(|w| w[1].open_time - w[0].open_time == tf_ms)
}

pub struct BinanceFetcher;
impl BinanceFetcher {
    pub fn new() -> Self { BinanceFetcher }

    pub async fn klines(
        &self,
        symbol: &str,
        tf: &str,
        limit: Option<u16>,
        start: Option<u64>,
        end: Option<u64>,
    ) -> Result<Vec<KlineSummary>> {

        let market: FuturesMarket = Binance::new(None, None);
        let klines = market.klines(symbol, tf, limit, start, end).await;

        todo!("Call your real klines API here")
    }
}

pub struct ClickhouseWriter;
impl ClickhouseWriter {
    pub fn new() -> Self { ClickhouseWriter }

    pub async fn write_batch(&self, klines: &[KlineSummary]) {
        // todo 需要新增实现批量插入函数
        // get_ck_db().insert(klines).await.unwrap();
        todo!("Insert into ClickHouse here")

    }
}


/// 这是一个用于历史数据进行清理、归档的函数
///
/// # 参数
/// - `symbol`: 交易对名称
/// - `exchange`: 交易所名称
/// - `tf`: 时间周期
///
async fn historical_maintenance_process(symbol: String, exchange: String, tf: TimeFrame) {

    // 构造归档任务
    let task = ArchiveTask {
        symbol: "BTCUSDT".to_string(),
        exchange: "binance".to_string(),
        tf: TimeFrame::M1,
        window: ArchiveWindow {
            start_time: Some(1_685_000_000_000), // 手动指定归档起点（毫秒时间戳）TODO: 不设置默认取归档历史最新时间
            end_time: Some(Utc::now().timestamp_millis()), // 默认使用当前时间
        },
        direction: ArchiveDirection::Forward,
    };

    // 执行归档任务
    // if let Err(e) = run_archive_task(task).await {
    //     eprintln!("归档失败: {:?}", e);
    // } else {
    //     println!("归档完成");
    // }

    // 构造向后归档任务（从 end_time 向 start_time 回溯）todo 获取归档状态表最小时间 作为end_time 通过k线数【币安默认一次性返回1000条】，周期，end_time计算start_time
    let start_time = 1_685_000_000_000; // 手动指定归档起点（毫秒时间戳）
    let end_time = Utc::now().timestamp_millis(); // 默认使用当前时间
    let task = ArchiveTask {
        symbol: "BTCUSDT".into(),
        exchange: "binance".into(),
        tf: TimeFrame::M1,
        window: ArchiveWindow {
            start_time: Some(start_time),
            end_time: Some(end_time),
        },
        direction: ArchiveDirection::Backward,
    };

    // // 执行归档
    // match run_archive_task(task).await {
    //     Ok(_) => println!("归档成功"),
    //     Err(e) => eprintln!("归档失败: {:?}", e),
    // }
}