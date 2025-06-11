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
    use chrono::{Duration, Utc};

    let now = Utc::now();
    let offset_dt = now.checked_sub_signed(Duration::days(days_offset))?;
    let offset_ms = offset_dt.timestamp_millis();

    let tf_ms = tf.to_millis();
    if tf_ms == 0 {
        return None; // 避免除零
    }

    // 对齐到时间周期的起点
    let aligned_start = offset_ms - (offset_ms % tf_ms);

    // 只能返回早于当前时间的时间戳
    if aligned_start < now.timestamp_millis() {
        Some(aligned_start)
    } else {
        None
    }
}

/// 对齐时间戳到周期起点
fn align_to_period_start(ts: i64, period_ms: i64) -> i64 {
    ts - (ts % period_ms)
}

/// 创建安全时间窗口，适用于回溯方向
/// start_time 应为较早时间点，end_time 应为较晚时间点
/// chunk_size 为单个窗口大小，period_ms 为时间周期长度
pub fn create_aligned_windows_with_limit_backward(
    start_time: i64,
    end_time: i64,
    chunk_size: i64,
    period_ms: i64,
) -> Vec<ArchiveWindow> {
    assert!(chunk_size > 0, "chunk_size must be positive");
    assert!(period_ms > 0, "period_ms must be positive");

    let aligned_start = align_to_period_start(start_time, period_ms);
    let aligned_end = align_to_period_start(end_time, period_ms);

    let safe_start = if aligned_start < 0 { 0 } else { aligned_start };

    if safe_start >= aligned_end {
        return Vec::new();
    }

    split_into_chunks(safe_start, aligned_end, chunk_size)
        .into_iter()
        .map(|(s, e)| ArchiveWindow {
            start_time: Some(s),
            end_time: Some(e),
        })
        .collect()
}

/// 生成时间窗口，自动限制窗口范围不超过当前时间对齐起点，
/// 若无有效时间区间返回空vec
pub fn create_aligned_windows_with_limit(
    start_time: i64,
    end_time: i64,
    chunk_size: i64,
    period_ms: i64,
) -> Vec<ArchiveWindow> {
    assert!(chunk_size > 0, "chunk_size must be positive");
    assert!(period_ms > 0, "period_ms must be positive");

    let aligned_start = align_to_period_start(start_time, period_ms);
    let now_aligned = align_to_period_start(Utc::now().timestamp_millis(), period_ms);

    let safe_start = if aligned_start < 0 { 0 } else { aligned_start };

    let limited_end = end_time.min(now_aligned);

    if limited_end <= safe_start {
        return Vec::new();
    }

    split_into_chunks(safe_start, limited_end, chunk_size)
        .into_iter()
        .map(|(s, e)| ArchiveWindow {
            start_time: Some(s),
            end_time: Some(e),
        })
        .collect()
}

pub fn split_into_chunks(start_time: i64, end_time: i64, chunk_size: i64) -> Vec<(i64, i64)> {
    assert!(chunk_size > 0, "chunk_size must be positive");
    let mut result = vec![];
    let mut current = start_time;
    while current < end_time {
        let next = (current + chunk_size).min(end_time);
        result.push((current, next));
        current = next;
    }
    result
}
