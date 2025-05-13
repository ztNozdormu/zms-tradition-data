use crate::db::ckdb::Database;
use crate::global::{get_ck_db, get_futures_market};
use crate::model::TimeFrame;
use crate::model::cex::kline::{MarketKline, MinMaxCloseTime};
use anyhow::Result;
use backoff::{ExponentialBackoff, future::retry};
/// This file contains the implementation of the maintenance module of the trade consumer.
/// 对历史数据进行清理、归档、缓存等操作
use barter::barter_xchange::exchange::binance::api::Binance;
use barter::barter_xchange::exchange::binance::futures::market::FuturesMarket;
use barter::barter_xchange::exchange::binance::model::{KlineSummaries, KlineSummary};
use chrono::{DateTime, Utc};
use clickhouse::Row;
use futures_util::TryFutureExt;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;
use tracing::{error, info, warn};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Copy)]
#[repr(i8)]
pub enum ArchiveDirection {
    Forward = 1,
    Backward = 2,
}

impl ArchiveDirection {
    pub fn as_str(&self) -> &'static str {
        match self {
            ArchiveDirection::Forward => "forward",
            ArchiveDirection::Backward => "backward",
        }
    }
    // 将 ArchiveDirection 转换为 i8
    pub fn to_i8(&self) -> i8 {
        *self as i8
    }

    // 将 i8 转换为 ArchiveDirection
    pub fn from_i8(value: i8) -> Option<ArchiveDirection> {
        match value {
            1 => Some(ArchiveDirection::Forward),
            2 => Some(ArchiveDirection::Backward),
            _ => None, // 不合法的值
        }
    }
}

// 定义 ArchiveError 枚举，涵盖不同类型的错误
#[derive(Debug)]
pub enum ArchiveError {
    NetworkError(String),
    DatabaseError(String),
    DataError(String),
    TimeoutError(String),
    OtherError(String),
}

// 实现 Display trait 来格式化错误消息
impl fmt::Display for ArchiveError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ArchiveError::NetworkError(msg) => write!(f, "Network Error: {}", msg),
            ArchiveError::DatabaseError(msg) => write!(f, "Database Error: {}", msg),
            ArchiveError::DataError(msg) => write!(f, "Data Error: {}", msg),
            ArchiveError::TimeoutError(msg) => write!(f, "Timeout Error: {}", msg),
            ArchiveError::OtherError(msg) => write!(f, "Other Error: {}", msg),
        }
    }
}

// 实现 From trait 来支持转换其他错误类型为 ArchiveError
impl From<std::io::Error> for ArchiveError {
    fn from(error: std::io::Error) -> Self {
        ArchiveError::NetworkError(error.to_string())
    }
}

impl From<tokio::time::error::Elapsed> for ArchiveError {
    fn from(error: tokio::time::error::Elapsed) -> Self {
        ArchiveError::TimeoutError(error.to_string())
    }
}

// ========== Input Structures ==========

#[derive(Debug, Clone)]
pub struct ArchiveTask {
    pub symbol: String,
    pub exchange: String,
    pub tf: Arc<TimeFrame>,
    pub window: ArchiveWindow,
    pub direction: ArchiveDirection,
}

#[derive(Debug, Clone)]
pub struct ArchiveWindow {
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
}

// ========== Archive Progress Table Logic ==========

pub struct ProgressTracker;

impl ProgressTracker {
    pub async fn get_progress(symbol: &str, exchange: &str, tf: &str) -> Option<MinMaxCloseTime> {
        match get_ck_db().get_mima_time(exchange, symbol, tf).await {
            Ok(Some(record)) => Some(MinMaxCloseTime {
                min_close_time: record.min_close_time,
                max_close_time: record.max_close_time,
            }),
            Ok(None) => None,
            Err(err) => {
                error!(?err, "Failed to get archive progress");
                None
            }
        }
    }
}
// ========== Main Archive Logic ==========

pub async fn run_archive_task(tasks: &Vec<ArchiveTask>) -> Result<(), ArchiveError> {
    for task in tasks {
        let tf_str = task.tf.to_str();
        let tf_ms = task.tf.to_period(); // 每根K线的毫秒数

        // === 起始时间逻辑 ===
        let mut current_time: i64 = match task.window.start_time {
            Some(start) => start,
            None => {
                warn!("Task has no start_time defined. Skipping.");
                continue;
            }
        };

        let end_time = task
            .window
            .end_time
            .unwrap_or_else(|| Utc::now().timestamp_millis());

        let fetcher = BinanceFetcher::new();
        let writer = ClickhouseWriter::new();

        info!(
            "Starting archive task: {} {} [{} ~ {}] direction={:?}",
            task.exchange, task.symbol, current_time, end_time, task.direction
        );

        // === 主归档循环 ===
        while match task.direction {
            ArchiveDirection::Forward => current_time < end_time,
            ArchiveDirection::Backward => current_time > end_time,
        } {
            // 计算一个批次的时间范围
            let next_time = advance_time(current_time, tf_ms * 1000, task.direction);
            let (start, end) = match next_time {
                Some(t) => match task.direction {
                    ArchiveDirection::Forward => (current_time, t),
                    ArchiveDirection::Backward => (t, current_time),
                },
                None => {
                    warn!("Time computation overflowed. Ending archive.");
                    break;
                }
            };

            // === 拉取数据（含重试） ===
            let klines = retry(ExponentialBackoff::default(), || async {
                fetcher
                    .klines(
                        &task.symbol,
                        tf_str,
                        Some(1000),
                        Some(start as u64),
                        Some(end as u64),
                    )
                    .await
                    .map_err(|e| {
                        warn!(?e, "Failed to fetch Klines, retrying...");
                        backoff::Error::transient(e)
                    })
            })
            .await
            .map_err(|e| ArchiveError::DataError(e.to_string()))?;

            if klines.is_empty() {
                info!("No Kline data between {} ~ {}. Stopping.", start, end);
                break;
            }

            // 检查连续性
            if !is_kline_continuous(&klines, tf_ms) {
                warn!("Kline gap detected in range {} ~ {}", start, end);
                // 可选扩展：记录 gap，打标，补全等
            }

            // === 写入 ClickHouse ===
            writer
                .write_batch(&klines, &task.exchange, &task.symbol, tf_str)
                .await
                .map_err(|e| ArchiveError::DatabaseError(e.to_string()))?;

            // === 推进 current_time ===
            let last_ts = klines
                .iter()
                .map(|k| k.close_time as i64)
                .max()
                .unwrap_or(current_time);

            current_time = advance_time(last_ts, tf_ms, task.direction).unwrap_or_else(|| {
                warn!("Failed to advance current_time (overflow). Ending task.");
                end_time
            });
        }

        info!(
            "Archive task finished: {} {} [{:?}]",
            task.exchange, task.symbol, task.direction
        );
    }

    Ok(())
}

/// 安全推进时间：根据方向进行加/减，防止溢出
fn advance_time(current: i64, delta: i64, direction: ArchiveDirection) -> Option<i64> {
    match direction {
        ArchiveDirection::Forward => current.checked_add(delta),
        ArchiveDirection::Backward => current.checked_sub(delta),
    }
}

// ========== Helper & Placeholder Stubs ==========

fn is_kline_continuous(klines: &[KlineSummary], tf_ms: i64) -> bool {
    klines
        .windows(2)
        .all(|w| w[1].open_time - w[0].open_time == tf_ms)
}

pub struct KlineContext<'a> {
    pub summary: &'a KlineSummary,
    pub exchange: String,
    pub symbol: String,
    pub period: String,
}
impl<'a> From<KlineContext<'a>> for MarketKline {
    fn from(ctx: KlineContext<'a>) -> Self {
        let s = ctx.summary;
        MarketKline {
            exchange: ctx.exchange,
            symbol: ctx.symbol,
            period: ctx.period,

            open_time: s.open_time,
            open: s.open,
            high: s.high,
            low: s.low,
            close: s.close,
            volume: s.volume,
            close_time: s.close_time,

            quote_asset_volume: s.quote_asset_volume,
            number_of_trades: s.number_of_trades as u64,
            taker_buy_base_asset_volume: s.taker_buy_base_asset_volume,
            taker_buy_quote_asset_volume: s.taker_buy_quote_asset_volume,
        }
    }
}

pub struct BinanceFetcher;
impl BinanceFetcher {
    pub fn new() -> Self {
        BinanceFetcher
    }

    pub async fn klines(
        &self,
        symbol: &str,
        tf: &str,
        limit: Option<u16>,
        start: Option<u64>,
        end: Option<u64>,
    ) -> Result<Vec<KlineSummary>> {
        let symbol_with_usdt = format!("{}usdt", symbol);
        // 请求 K线数据
        let summaries = get_futures_market()
            .klines(symbol_with_usdt, tf, limit, start, end)
            .await
            .unwrap();
        if let KlineSummaries::AllKlineSummaries(klines) = summaries {
            Ok(klines)
        } else {
            Ok(Vec::new())
        }
    }
}

pub struct ClickhouseWriter;
impl ClickhouseWriter {
    pub fn new() -> Self {
        ClickhouseWriter
    }

    pub async fn write_batch(
        &self,
        klines: &[KlineSummary],
        exchange: &str,
        symbol: &str,
        period: &str,
    ) -> Result<(), anyhow::Error> {
        if klines.is_empty() {
            return Ok(()); // 提前返回，避免不必要操作
        }

        // 批量构造 MarketKline
        let market_klines: Vec<MarketKline> = klines
            .iter()
            .map(|kline| {
                KlineContext {
                    summary: kline,
                    exchange: exchange.into(),
                    symbol: symbol.into(),
                    period: period.into(),
                }
                .into()
            })
            .collect();

        // 批量写入 ClickHouse
        get_ck_db().insert_batch(&market_klines).await?;

        Ok(())
    }
}

/// 这是一个用于历史数据进行清理、归档的函数
///
/// # 参数
/// - `symbol`: 交易对名称
/// - `exchange`: 交易所名称
/// - `tf`: 时间周期
pub async fn historical_maintenance_process(
    symbol: String,
    exchange: String,
    close_time: i64,
    time_frame: Arc<TimeFrame>,
) {
    // let time_frame = Arc::new(tf);
    // 获取历史记录最大时间和最小时间
    let mima_time =
        match ProgressTracker::get_progress(&symbol, &exchange, &time_frame.to_str()).await {
            Some(progress) => progress,
            None => {
                info!("No existing progress found, initializing the task.");

                // 如果没有进度数据，我们尝试使用某个默认的起始时间（例如过去的某个日期）
                let fallback_time =
                    get_default_start_time(close_time, &time_frame).unwrap_or_else(|| {
                        // 如果无法获取默认时间，则使用当前时间的某个合理的回退时间
                        Utc::now().timestamp_millis() - 86_400_000 // 一天前
                    });
                MinMaxCloseTime {
                    min_close_time: fallback_time,
                    max_close_time: 0,
                }
            }
        };

    // 根据mima_time 构建回溯和追溯任务
    // 创建归档任务（前向）max_close_time 与 close_time 间隔大于2个周期及以上才构建追溯任务
    //
    let forward_task = ArchiveTask {
        symbol: symbol.clone(),
        exchange: exchange.clone(),
        tf: time_frame.clone(),
        window: ArchiveWindow {
            start_time: Some(mima_time.max_close_time),
            end_time: Some(close_time),
        },
        direction: ArchiveDirection::Forward, // 默认归档方向为前向
    };

    let back_task = ArchiveTask {
        symbol: symbol.clone(),
        exchange: exchange.clone(),
        tf: time_frame.clone(),
        window: ArchiveWindow {
            start_time: Some(mima_time.max_close_time - 1000 * time_frame.to_period()), // 往历史记录回溯1000根K线得到回溯起始时间
            end_time: Some(mima_time.max_close_time),
        },
        direction: ArchiveDirection::Backward, // 默认归档方向为前向
    };
    let tasks = vec![forward_task, back_task];
    // 执行前向归档任务，并加入重试机制
    if let Err(e) = run_archive_task_with_retry(&tasks).await {
        error!(?e, "Failed to execute forward archive task");
        // 失败后可以考虑通知机制，如通过 Webhook 或邮件通知管理员
    } else {
        info!(
            "Forward archive task completed for {} - {} - {}",
            symbol,
            exchange,
            &time_frame.to_str()
        );
    }
}

async fn run_archive_task_with_retry(tasks: &Vec<ArchiveTask>) -> Result<(), ArchiveError> {
    const MAX_RETRIES: u8 = 3;
    const RETRY_DELAY: Duration = Duration::from_secs(5);

    let mut retries = 0;
    loop {
        match run_archive_task(tasks).await {
            Ok(_) => return Ok(()),
            Err(e) => {
                if retries >= MAX_RETRIES {
                    return Err(e); // 达到最大重试次数，返回错误
                }
                retries += 1;
                error!(?e, "Archive task failed, retrying...");
                sleep(RETRY_DELAY).await; // 等待 5 秒再重试
            }
        }
    }
}

/// 获取默认起始时间（如果有配置或其他数据源）
fn get_default_start_time(close_time: i64, tf: &TimeFrame) -> Option<i64> {
    // 获取时间周期的毫秒数
    let tf_ms = tf.to_period(); // 假设 `to_period()` 返回该周期的毫秒数

    // 计算一个周期之前的时间点，排除掉已计算的最新K线
    let start_time = close_time - tf_ms;

    // 确保返回的时间不>于当前时间，避免归档时间超出范围
    let now = Utc::now().timestamp_millis();
    if start_time < now {
        Some(start_time)
    } else {
        None
    }
}
