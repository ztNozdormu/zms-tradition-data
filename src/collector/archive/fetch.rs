pub mod helper;
pub mod progress;

use crate::collector::archive::fetch::helper::{
    create_windows, is_kline_continuous, should_skip_archiving_due_to_old_data, valid_window_range,
};
use crate::collector::archive::fetch::progress::ProgressTracker;
use crate::collector::archive::types::{ArchiveDirection, ArchiveError, ArchiveTask};
use crate::collector::archive::KlineMessage;
use crate::global::get_binance_limiter;
use crate::infra::external::binance::market::KlineSummary;
use crate::infra::external::binance::DefaultBinanceExchange;
use crate::model::cex::kline::MinMaxCloseTime;
use crate::model::TimeFrame;
use async_trait::async_trait;
use backoff::{future::retry, ExponentialBackoff};
use chrono::Utc;
use diesel::IntoSql;
use listen_tracing::trace_kv;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;
use tracing::{error, info, warn};

#[async_trait]
pub trait KlineFetcher: Send + Sync {
    async fn klines(
        &self,
        symbol: &str,
        interval: &str,
        limit: Option<u16>,
        start_time: Option<u64>,
        end_time: Option<u64>,
    ) -> anyhow::Result<Vec<KlineSummary>, anyhow::Error>;
}

pub struct BinanceFetcher;

impl BinanceFetcher {
    pub fn new() -> Self {
        BinanceFetcher
    }
}

#[async_trait]
impl KlineFetcher for BinanceFetcher {
    async fn klines(
        &self,
        symbol: &str,
        tf: &str,
        limit: Option<u16>,
        start: Option<u64>,
        end: Option<u64>,
    ) -> anyhow::Result<Vec<KlineSummary>> {
        // 阻塞式限流，等待令牌
        get_binance_limiter().acquire_with_limit(limit.unwrap_or(1000).into()).await;
        //let symbol_with_usdt = format!("{}usdt", symbol);
        let dbe = DefaultBinanceExchange::default();
        let klines = dbe.get_klines(symbol, tf, limit, start, end).await;
        Ok(klines)
    }
}

/// 顶层调度：用于定时器、外部调用等
pub async fn kline_fetch_process(
    symbol: String,
    exchange: String,
    time_frame: Arc<TimeFrame>,
) -> Vec<KlineMessage> {
    let tf_str = time_frame.to_str();

    // 3. 构建归档任务
    let tasks = build_all_archive_tasks(&symbol, &exchange, time_frame.clone()).await;

    // 4. 执行带重试的归档任务
    match run_archive_task_with_retry(&tasks).await {
        Ok(messages) => {
            if messages.is_empty() {
                info!("No archive data for {} - {} - {}", symbol, exchange, tf_str);
            } else {
                info!(
                    "Archived {} chunks for {} - {} - {}",
                    messages.len(),
                    symbol,
                    exchange,
                    tf_str
                );
            }
            messages
        }
        Err(e) => {
            error!(?e, "Failed to complete archive task, returning empty.");
            vec![]
        }
    }
}

/// 构建双向任务
pub async fn build_all_archive_tasks(
    symbol: &str,
    exchange: &str,
    tf: Arc<TimeFrame>,
) -> Vec<ArchiveTask> {
    let mut tasks = vec![];

    if let Some(backward_task) = build_backward_tasks(symbol, exchange, tf.clone()).await {
        tasks.push(backward_task);
    }

    if let Some(forward_task) = build_forward_tasks(symbol, exchange, tf.clone()).await {
        tasks.push(forward_task);
    }

    tasks
}

/// 构建回溯任务（Backward）
pub async fn build_backward_tasks(
    symbol: &str,
    exchange: &str,
    tf: Arc<TimeFrame>,
) -> Option<ArchiveTask> {
    let mima_time =
        ProgressTracker::get_or_init_progress(symbol, exchange, &tf, ArchiveDirection::Backward)
            .await;

    // 2. 数据太老跳过
    if should_skip_archiving_due_to_old_data(mima_time.min_close_time, &symbol, &exchange, &tf) {
        info!(
            "Skipping old data: {} - {} - {}",
            symbol,
            exchange,
            tf.to_str()
        );
        return None;
    }

    let period_ms = tf.to_millis();
    let backtrack_count = tf.backtrack_count() as i64;
    let backtrack_ms = backtrack_count * period_ms;
    let default_chunk_size_ms = 1000 * period_ms;
    let actual_chunk_size_ms = default_chunk_size_ms.min(backtrack_ms);

    let start = mima_time.min_close_time - backtrack_ms;
    let end = mima_time.min_close_time;

    if start < end {
        Some(ArchiveTask {
            symbol: symbol.to_string(),
            exchange: exchange.to_string(),
            tf,
            window: create_windows(start, end, actual_chunk_size_ms),
            direction: ArchiveDirection::Backward,
        })
    } else {
        None
    }
}

/// 构建追溯任务（Forward）
pub async fn build_forward_tasks(
    symbol: &str,
    exchange: &str,
    tf: Arc<TimeFrame>,
) -> Option<ArchiveTask> {
    let mima_time =
        ProgressTracker::get_or_init_progress(symbol, exchange, &tf, ArchiveDirection::Forward)
            .await;

    let period_ms = tf.to_millis();
    let backtrack_count = tf.backtrack_count() as i64;
    let backtrack_ms = backtrack_count * period_ms;

    // 默认1000根[binance]一批次
    let default_chunk_size_ms = 1000 * period_ms;

    // 保证 chunk_size 不超过总回溯区间
    let actual_chunk_size_ms = default_chunk_size_ms.min(backtrack_ms);

    let forward_end = mima_time.max_close_time + backtrack_ms;

    Some(ArchiveTask {
        symbol: symbol.to_string(),
        exchange: exchange.to_string(),
        tf,
        window: create_windows(mima_time.max_close_time, forward_end, actual_chunk_size_ms),
        direction: ArchiveDirection::Forward,
    })
}

/// 封装归档任务重试逻辑
pub async fn run_archive_task_with_retry(
    tasks: &[ArchiveTask],
) -> Result<Vec<KlineMessage>, ArchiveError> {
    const MAX_RETRIES: u8 = 3;
    const RETRY_DELAY: Duration = Duration::from_secs(5);
    let mut retries = 0;

    loop {
        match execute_archive_messages(tasks).await {
            Ok(messages) => return Ok(messages),
            Err(e) => {
                retries += 1;
                error!(
                    ?e,
                    "Archive task failed, attempt {}/{}", retries, MAX_RETRIES
                );

                if retries >= MAX_RETRIES {
                    return Err(e);
                }

                sleep(RETRY_DELAY).await;
            }
        }
    }
}

///  execute archive task fetch kline_message
pub async fn execute_archive_messages(
    tasks: &[ArchiveTask],
) -> Result<Vec<KlineMessage>, ArchiveError> {
    let fetcher = BinanceFetcher::new();
    let mut messages = Vec::with_capacity(tasks.len() * 2); // 预估容量

    for task in tasks {
        let tf_str = task.tf.to_str();
        let tf_ms = task.tf.to_millis();

        trace_kv!(info,
             "exchange" => task.exchange,
             "symbol" => task.symbol,
             "windows" => task.window.len(),
        );

        for window in task.window.iter().filter_map(valid_window_range) {
            let klines = fetch_klines_with_retry(
                &fetcher,
                &task.symbol,
                tf_str,
                window.start_time.unwrap(),
                window.end_time.unwrap(),
            )
            .await?;

            if klines.is_empty() {
                info!(
                    "No Kline data between {} ~ {}.",
                    window.start_time.unwrap_or(0),
                    window.end_time.unwrap_or(0)
                );
                continue;
            }

            if !is_kline_continuous(&klines, tf_ms) {
                warn!(
                    "Kline gap detected in range {} ~ {}",
                    window.start_time.unwrap_or(0),
                    window.end_time.unwrap_or(0)
                );
            }

            let mut sorted = klines;
            sorted.sort_by_key(|k| k.close_time);

            messages.push(KlineMessage {
                datas: sorted,
                symbol: task.symbol.clone(),
                exchange: task.exchange.clone(),
                time_frame: tf_str.to_string(),
                archive_direction: task.direction,
            });
        }
    }

    Ok(messages)
}

/// 使用 backoff 拉取 K线
async fn fetch_klines_with_retry(
    fetcher: &impl KlineFetcher,
    symbol: &str,
    tf_str: &str,
    start: i64,
    end: i64,
) -> Result<Vec<KlineSummary>, ArchiveError> {
    retry(ExponentialBackoff::default(), || async {
        fetcher
            .klines(
                symbol,
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
    .map_err(|e| ArchiveError::DataError(e.to_string()))
}
