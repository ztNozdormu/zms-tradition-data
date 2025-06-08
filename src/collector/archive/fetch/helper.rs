use crate::collector::archive::types::ArchiveWindow;
use crate::infra::external::binance::market::KlineSummary;
use crate::model::TimeFrame;
use chrono::{Duration, Utc};
use tracing::{info, warn};

// ========== Helper & Placeholder Stubs ==========

pub fn is_kline_continuous(klines: &[KlineSummary], tf_ms: i64) -> bool {
    klines
        .windows(2)
        .all(|w| w[1].close_time - w[0].close_time == tf_ms)
}

/// 检查时间窗口是否合法
pub fn valid_window_range(window: &ArchiveWindow) -> Option<ArchiveWindow> {
    match (window.start_time, window.end_time) {
        (Some(start_time), Some(end_time)) if start_time < end_time => Some(ArchiveWindow {
            start_time: Some(start_time),
            end_time: Some(end_time),
        }),
        _ => {
            warn!(
                "Invalid or missing time window: start={:?}, end={:?}",
                window.start_time, window.end_time
            );
            None
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

/// 默认起点：当前时间往前推三个月，并对齐到对应时间周期的起点
pub fn get_default_start_time_with_offset(tf: &TimeFrame, days_offset: i64) -> Option<i64> {
    let now = Utc::now().timestamp_millis();
    let offset_time = now - Duration::days(days_offset).num_milliseconds();
    let tf_ms = tf.to_millis();
    let aligned_start = offset_time - (offset_time % tf_ms);
    if aligned_start < now {
        Some(aligned_start)
    } else {
        None
    }
}

/// 时间窗口生成
pub fn create_windows(start_time: i64, end_time: i64, chunk_size: i64) -> Vec<ArchiveWindow> {
    crate::collector::maintenance::split_into_chunks(start_time, end_time, chunk_size)
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
