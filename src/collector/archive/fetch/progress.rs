use crate::collector::archive::fetch::helper::get_default_start_time;
use crate::global::get_ck_db;
use crate::model::cex::kline::MinMaxCloseTime;
use crate::model::TimeFrame;
use chrono::Utc;
use tracing::{error, info};

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

    /// 初始化进度或设置默认起点
    pub async fn fetch_or_initialize_progress(
        symbol: &str,
        exchange: &str,
        tf_str: &str,
        close_time: i64,
        time_frame: &TimeFrame,
    ) -> MinMaxCloseTime {
        match ProgressTracker::get_progress(symbol, exchange, tf_str).await {
            Some(progress) => progress,
            None => {
                info!(
                    "No progress found. Using fallback time for {} - {} - {}",
                    symbol, exchange, tf_str
                );
                let fallback = get_default_start_time(close_time, time_frame)
                    .unwrap_or_else(|| Utc::now().timestamp_millis() - 86_400_000); // 默认：一天前
                MinMaxCloseTime {
                    min_close_time: fallback,
                    max_close_time: 0,
                }
            }
        }
    }
}
