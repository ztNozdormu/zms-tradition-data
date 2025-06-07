mod types;

use crate::collector::maintenance::types::{
    ArchiveDirection, ArchiveError, ArchiveTask, ArchiveWindow,
};
use crate::global::get_ck_db;
use crate::infra::db::types::ClickHouseDatabase;
use crate::infra::external::binance::market::KlineSummary;
use crate::infra::external::binance::DefaultBinanceExchange;
use crate::model::cex::kline::{MarketKline, MinMaxCloseTime};
use crate::model::TimeFrame;
use crate::scheduler::tasks::history_data::KlineMessage;
use anyhow::Result;
use backoff::{future::retry, ExponentialBackoff};
/// This file contains the implementation of the maintenance module of the trade consumer.
/// 对历史数据进行清理、归档、缓存等操作
use chrono::Utc;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;
use tracing::{error, info, warn};

// ========== Archive Progress Logic ==========
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

pub async fn run_archive_task(
    tasks: &[ArchiveTask],
    symbol: &str,
    exchange: &str,
    time_frame: &str,
) -> Result<(), ArchiveError> {
    let fetcher = BinanceFetcher::new();
    let writer = ClickhouseWriter::new();

    let mut all_klines = Vec::new();

    for task in tasks {
        let tf_str = task.tf.to_str();
        let tf_ms = task.tf.to_millis();

        info!(
            "Starting archive task: {} {} direction={:?}, windows={}",
            task.exchange,
            task.symbol,
            task.direction,
            task.window.len()
        );

        for window in &task.window {
            let start = match window.start_time {
                Some(t) => t,
                None => {
                    warn!("Missing start_time in window. Skipping.");
                    continue;
                }
            };

            let end = match window.end_time {
                Some(t) => t,
                None => {
                    warn!("Missing end_time in window. Skipping.");
                    continue;
                }
            };

            if start >= end {
                warn!(
                    "Invalid window: start >= end [{} >= {}]. Skipping.",
                    start, end
                );
                continue;
            }

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
                info!("No Kline data between {} ~ {}.", start, end);
                continue;
            }

            // === 连续性检查 ===
            if !is_kline_continuous(&klines, tf_ms) {
                warn!("Kline gap detected in range {} ~ {}", start, end);
            }

            // 批量构造 MarketKline
            let market_klines: Vec<MarketKline> = klines
                .iter()
                .map(|k| (k, &task.exchange[..], &task.symbol[..], tf_str).into())
                .collect();

            all_klines.extend(market_klines);
        }
    }
    if all_klines.is_empty() {
        info!(
            "No Klines collected for task {} {} [{:?}]",
            exchange, symbol, time_frame
        );
        return Ok(()); // 提前返回，避免不必要操作
    }
    // 按时间排序以提升 ClickHouse 插入性能（可选）
    all_klines.sort_by_key(|k| k.close_time);

    // TODO all_klines 当批次任务处理数据量如果小于1000条则先放入缓存，延迟/累积入库
    // TODO get_flush_controller().push(exchange,symbol,tf.to_str(),SEGMENT_RECENT,&market_kline).await

    // === 批量写入 ClickHouse ===
    writer
        .write_batch(&all_klines)
        .await
        .map_err(|e| ArchiveError::DatabaseError(e.to_string()))?;

    info!(
        "Archive task finished: {} {} [{:?}], total klines: {}",
        exchange,
        symbol,
        time_frame,
        all_klines.len()
    );

    Ok(())
}

pub async fn generate_archive_messages(
    tasks: &[ArchiveTask],
) -> Result<Vec<KlineMessage>, ArchiveError> {
    let fetcher = BinanceFetcher::new();

    let mut messages = Vec::new();

    for task in tasks {
        let tf_str = task.tf.to_str();
        let tf_ms = task.tf.to_millis();

        info!(
            "Starting archive task: {} {} direction={:?}, windows={}",
            task.exchange,
            task.symbol,
            task.direction,
            task.window.len()
        );

        for window in &task.window {
            let start = match window.start_time {
                Some(t) => t,
                None => {
                    warn!("Missing start_time in window. Skipping.");
                    continue;
                }
            };

            let end = match window.end_time {
                Some(t) => t,
                None => {
                    warn!("Missing end_time in window. Skipping.");
                    continue;
                }
            };

            if start >= end {
                warn!(
                    "Invalid window: start >= end [{} >= {}]. Skipping.",
                    start, end
                );
                continue;
            }

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
                info!("No Kline data between {} ~ {}.", start, end);
                continue;
            }

            // === 连续性检查 ===
            if !is_kline_continuous(&klines, tf_ms) {
                warn!("Kline gap detected in range {} ~ {}", start, end);
            }

            // 封装为 KlineMessage（用于 downstream 阶段入缓存或批量入库）
            let message = KlineMessage {
                datas: klines,
                symbol: task.symbol.clone(),
                exchange: task.exchange.clone(),
                time_frame: tf_str.to_string(),
            };

            messages.push(message);
        }
    }

    Ok(messages)
}
// ========== Helper & Placeholder Stubs ==========

fn is_kline_continuous(klines: &[KlineSummary], tf_ms: i64) -> bool {
    klines
        .windows(2)
        .all(|w| w[1].close_time - w[0].close_time == tf_ms)
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
        let dbe = DefaultBinanceExchange::default();
        let klines = dbe
            .get_klines(symbol_with_usdt, tf, limit, start, end)
            .await;
        Ok(klines)
    }
}

pub struct ClickhouseWriter;
impl ClickhouseWriter {
    pub fn new() -> Self {
        ClickhouseWriter
    }

    pub async fn write_batch(&self, market_klines: &[MarketKline]) -> Result<(), anyhow::Error> {
        if market_klines.is_empty() {
            return Ok(()); // 提前返回，避免不必要操作
        }
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
    // info!("is max_close is {} min close is {}",&mima_time.max_close_time,&mima_time.min_close_time);
    // 默认只归档不超过五年的数据
    if should_skip_archiving_due_to_old_data(
        mima_time.min_close_time,
        &symbol,
        &exchange,
        &time_frame,
    ) {
        return;
    }
    // 构建回溯/追溯任务集合
    let tasks = build_archive_tasks(
        &symbol,
        &exchange,
        time_frame.clone(),
        &mima_time,
        close_time,
    );

    // 执行前向归档任务，并加入重试机制
    if let Err(e) =
        run_archive_task_with_retry(&tasks, &symbol, &exchange, &time_frame.to_str()).await
    {
        error!(?e, "Failed to execute archive task");
        // 失败后可以考虑通知机制，如通过 Webhook 或邮件通知管理员
    } else {
        info!(
            "archive task completed for {} - {} - {}",
            symbol,
            exchange,
            &time_frame.to_str()
        );
    }
}

pub async fn historical_maintenance_backforward_process(
    symbol: String,
    exchange: String,
    close_time: i64,
    time_frame: Arc<TimeFrame>,
) {
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
    // info!("is max_close is {} min close is {}",&mima_time.max_close_time,&mima_time.min_close_time);
    // 默认只归档不超过五年的数据
    if should_skip_archiving_due_to_old_data(
        mima_time.min_close_time,
        &symbol,
        &exchange,
        &time_frame,
    ) {
        return;
    }
    // 构建回溯/追溯任务集合
    let tasks = build_archive_tasks(
        &symbol,
        &exchange,
        time_frame.clone(),
        &mima_time,
        close_time,
    );

    // 执行前向归档任务，并加入重试机制
    if let Err(e) =
        run_archive_task_with_retry(&tasks, &symbol, &exchange, &time_frame.to_str()).await
    {
        error!(?e, "Failed to execute archive task");
        // 失败后可以考虑通知机制，如通过 Webhook 或邮件通知管理员
    } else {
        info!(
            "archive task completed for {} - {} - {}",
            symbol,
            exchange,
            &time_frame.to_str()
        );
    }
}

async fn run_archive_task_with_retry(
    tasks: &Vec<ArchiveTask>,
    symbol: &str,
    exchange: &str,
    time_frame: &str,
) -> Result<(), ArchiveError> {
    const MAX_RETRIES: u8 = 3;
    const RETRY_DELAY: Duration = Duration::from_secs(5);

    let mut retries = 0;
    loop {
        match run_archive_task(tasks, symbol, exchange, time_frame).await {
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

/// 如果历史数据早于当前时间五年前，认为数据已经足够完整，可跳过归档。
pub fn should_skip_archiving_due_to_old_data(
    min_close_time: i64,
    symbol: &str,
    exchange: &str,
    tf: &TimeFrame,
) -> bool {
    let five_years_ago = Utc::now().timestamp_millis() - (5 * 365 * 24 * 60 * 60 * 1000);
    if min_close_time <= five_years_ago {
        info!(
            "Min close time {:?} is earlier than 5 years ago, skipping archive for {} - {} - {}",
            min_close_time,
            symbol,
            exchange,
            tf.to_str()
        );
        true
    } else {
        false
    }
}

/// 获取默认起始时间（如果有配置或其他数据源）
fn get_default_start_time(close_time: i64, tf: &TimeFrame) -> Option<i64> {
    // 计算一个周期之前的时间点，排除掉已计算的最新K线
    let start_time = close_time;

    // 确保返回的时间不>于当前时间，避免归档时间超出范围
    let now = Utc::now().timestamp_millis();
    if start_time < now {
        Some(start_time)
    } else {
        None
    }
}

/// 构建回溯与追溯的归档任务集合
pub fn build_archive_tasks(
    symbol: &str,
    exchange: &str,
    time_frame: Arc<TimeFrame>,
    mima_time: &MinMaxCloseTime,
    close_time: i64,
) -> Vec<ArchiveTask> {
    let period_ms = time_frame.to_millis();
    let backtrack_count = time_frame.backtrack_count() as i64;
    let backtrack_ms = backtrack_count * period_ms;

    let default_chunk_size_ms = 1000 * period_ms;
    // 保证 chunk_size 不超过总回溯区间
    let actual_chunk_size_ms = default_chunk_size_ms.min(backtrack_ms);

    let mut tasks = vec![];

    // 构造任务的闭包函数
    let try_build_task =
        |direction: ArchiveDirection, start: i64, end: i64| -> Option<ArchiveTask> {
            if start < end {
                Some(ArchiveTask {
                    symbol: symbol.to_string(),
                    exchange: exchange.to_string(),
                    tf: time_frame.clone(),
                    window: create_windows(start, end, actual_chunk_size_ms),
                    direction,
                })
            } else {
                None
            }
        };

    // === Backward 任务 ===
    let backward_start = mima_time.min_close_time - backtrack_ms; // 得到追溯开始时间
    let backward_end = mima_time.min_close_time;
    if let Some(task) = try_build_task(ArchiveDirection::Backward, backward_start, backward_end) {
        tasks.push(task);
    }

    // === Forward 任务 ===
    if mima_time.max_close_time != 0 {
        let forward_start = mima_time.max_close_time;
        let forward_end = close_time; // redis缓存如果存在取最小时间
        if let Some(task) = try_build_task(ArchiveDirection::Forward, forward_start, forward_end) {
            tasks.push(task);
        }
    }

    tasks
}

/// 时间窗口生成
fn create_windows(start_time: i64, end_time: i64, chunk_size: i64) -> Vec<ArchiveWindow> {
    split_into_chunks(start_time, end_time, chunk_size)
        .into_iter()
        .map(|(s, e)| ArchiveWindow {
            start_time: Some(s),
            end_time: Some(e),
        })
        .collect()
}

pub fn split_into_chunks(start_time: i64, end_time: i64, chunk_size: i64) -> Vec<(i64, i64)> {
    let mut result = vec![];
    let mut current = start_time;
    while current < end_time {
        let next = (current + chunk_size).min(end_time);
        result.push((current, next));
        current = next;
    }
    result
}

/// 构造 Forward(追溯数据) 任务（从 max_close_time 向最新时间方向补齐到 close_time）
pub fn build_forward_task(
    symbol: &str,
    exchange: &str,
    time_frame: Arc<TimeFrame>,
    max_close_time: i64,
    close_time: i64,
) -> Option<ArchiveTask> {
    if max_close_time == 0 || max_close_time >= close_time {
        return None;
    }

    let period_ms = time_frame.to_millis();
    let default_chunk_size_ms = 1000 * period_ms;

    Some(ArchiveTask {
        symbol: symbol.to_string(),
        exchange: exchange.to_string(),
        tf: time_frame.clone(),
        window: create_windows(max_close_time, close_time, default_chunk_size_ms),
        direction: ArchiveDirection::Forward,
    })
}

/// 构造 Backward(回溯数据)（从 min_close_time 向历史时间回退）
pub fn build_backward_task(
    symbol: &str,
    exchange: &str,
    time_frame: Arc<TimeFrame>,
    min_close_time: i64,
) -> Option<ArchiveTask> {
    let period_ms = time_frame.to_millis();
    let backtrack_count = time_frame.backtrack_count() as i64;
    let backtrack_ms = backtrack_count * period_ms;

    let default_chunk_size_ms = 1000 * period_ms;
    let actual_chunk_size_ms = default_chunk_size_ms.min(backtrack_ms);

    let start = min_close_time - backtrack_ms;
    let end = min_close_time;

    if start < end {
        Some(ArchiveTask {
            symbol: symbol.to_string(),
            exchange: exchange.to_string(),
            tf: time_frame,
            window: create_windows(start, end, actual_chunk_size_ms),
            direction: ArchiveDirection::Backward,
        })
    } else {
        None
    }
}

/*
两级冷热分离存储设计
  思路:
      1. mysql存储最近三个月数据
      2. clickhouse存储历史数据
      3. mysql存储开始时间-》现在时间的数据--追溯
        3.1 获取开始时间
          3.1.1 查询历史最大时间,如果存在返回作为开始时间
                否则计算当前时间前一个周期k线close_time作为开始时间
        3.2 开始时间-》当前时间作为参数构造追溯任务处理数据入库
      4. clickhouse存储开始时间-》过去时间的数据--回溯
        4.1 获取开始时间
          4.1.1 查询历史最小记录时间,如果存在返回作为endTime(需要验证是否闭区间),
                结束时间往回倒推对应周期1000根(币安默认1000根)k线得到每一轮开始时间(endTime-time_frame*1000=startTime);
                否则查询mysql表记录最小时间作为结束时间(endTime).开始时间计算同上.
                **先决条件endTime时间不超过五年**
          4.1.2 在4.1.1时间参数逻辑基础上预先构造时间窗口区间(每个币1000条周期区间)
          4.1.3 多周期一个周期一个周期处理,每个周期循环中多个币历史k线回溯总量达到5w条(阈值),当轮返回的k线最小时间超过五年直接入库
*/
