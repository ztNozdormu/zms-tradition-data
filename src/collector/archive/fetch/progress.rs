use crate::collector::archive::fetch::helper::get_default_start_time_with_offset;
use crate::collector::archive::types::ArchiveDirection;
use crate::domain::repository::market_kline_repository::MarketKlineRepository;
use crate::domain::service::market_kline_service::MarketKlineService;
use crate::global::{get_ck_db, get_mysql_pool};
use crate::model::cex::kline::MinMaxCloseTime;
use crate::model::TimeFrame;
use chrono::Utc;
use tracing::{error, info};

pub struct ProgressTracker;

impl ProgressTracker {
    /// 获取归档进度，若无记录则使用默认时间初始化
    pub async fn get_or_init_progress(
        symbol: &str,
        exchange: &str,
        time_frame: &TimeFrame,
        direction: ArchiveDirection,
    ) -> MinMaxCloseTime {
        let progress_result = match direction {
            ArchiveDirection::Forward => {
                // 正确的异步调用
                match get_mysql_pool().get() {
                    Ok(mut conn) => {
                        let repo = MarketKlineRepository::new(&mut conn);
                        let mut service = MarketKlineService { repo };
                        match service
                            .get_mima_time(exchange, symbol, time_frame.to_str())
                            .await
                        {
                            Ok(Some(r)) => Some(MinMaxCloseTime {
                                min_close_time: r.min_close_time,
                                max_close_time: r.max_close_time,
                            }),
                            Ok(None) => None,
                            Err(err) => {
                                error!(?err, "MySQL: Failed to get archive progress");
                                None
                            }
                        }
                    }
                    Err(err) => {
                        error!(?err, "MySQL: Failed to get db connection");
                        None
                    }
                }
            }

            ArchiveDirection::Backward => {
                match get_ck_db()
                    .get_mima_time(exchange, symbol, time_frame.to_str())
                    .await
                {
                    Ok(Some(r)) => Some(MinMaxCloseTime {
                        min_close_time: r.min_close_time,
                        max_close_time: r.max_close_time,
                    }),
                    Ok(None) => None,
                    Err(err) => {
                        error!(?err, "ClickHouse: Failed to get archive progress");
                        None
                    }
                }
            }
        };

        match progress_result {
            Some(progress) => progress,
            None => {
                info!(
                    "No history progress found. Using fallback time for {} - {} - {}",
                    symbol,
                    exchange,
                    time_frame.to_str()
                );
                let fallback = get_default_start_time_with_offset(time_frame, 90)
                    .unwrap_or_else(|| Utc::now().timestamp_millis() - 86_400_000); // 默认一天前
                MinMaxCloseTime {
                    min_close_time: fallback,
                    max_close_time: fallback,
                }
            }
        }
    }
}
