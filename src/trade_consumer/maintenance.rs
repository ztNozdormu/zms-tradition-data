use crate::db::ckdb::Database;
use crate::global::{get_ck_db, get_futures_market};
use crate::model::TimeFrame;
use crate::model::cex::kline::{ArchiveDirection, ArchiveProgress, MarketKline};
use anyhow::Result;
use backoff::{ExponentialBackoff, future::retry};
/// This file contains the implementation of the maintenance module of the trade consumer.
/// 对历史数据进行清理、归档、缓存等操作
use barter::barter_xchange::exchange::binance::api::Binance;
use barter::barter_xchange::exchange::binance::futures::market::FuturesMarket;
use barter::barter_xchange::exchange::binance::model::{KlineSummaries, KlineSummary};
use chrono::{DateTime, Utc};
use futures_util::TryFutureExt;
use serde::Serialize;
use tracing::{error, info, warn};

// ========== Input Structures ==========

#[derive(Debug, Clone)]
pub struct ArchiveTask {
    pub symbol: String,
    pub exchange: String,
    pub tf: TimeFrame,
    pub window: ArchiveWindow,
    pub direction: ArchiveDirection,
}


#[derive(Debug, Clone)]
pub struct ArchiveWindow {
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
}

// ========== Archive Progress Table Logic ==========

/// 与 archive_progress 表结构对应的写入模型
#[derive(Debug, Serialize)]
struct ArchiveProgressRecord {
    exchange: String,
    symbol: String,
    period: String,
    direction: String, // "forward" or "backward"
    last_archived_time: u64,
    completed: u8,
    updated_at: chrono::NaiveDateTime,
}

pub struct ProgressTracker;

impl ProgressTracker {
    pub async fn get_progress(
        symbol: &str,
        exchange: &str,
        tf: &str,
        direction: &str,
    ) -> Option<ArchiveProgressRecord> {
        let dir = match direction {
            "forward" => ArchiveDirection::Forward,
            "backward" => ArchiveDirection::Backward,
            _ => {
                error!("Invalid direction: {}", direction);
                return None;
            }
        };

        match get_ck_db()
            .get_archive_progress(exchange, symbol, tf, dir)
            .await
        {
            Ok(Some(record)) => Some(ArchiveProgressRecord {
                exchange: record.exchange,
                symbol: record.symbol,
                period: record.period,
                direction: direction.to_string(),
                last_archived_time: record.last_archived_time,
                completed: record.completed,
                updated_at: record.updated_at,
            }),
            Ok(None) => None,
            Err(err) => {
                error!(?err, "Failed to get archive progress");
                None
            }
        }
    }

    pub async fn update_progress(progress: ArchiveProgressRecord) {
        let direction_enum = match progress.direction.as_str() {
            "forward" => ArchiveDirection::Forward,
            "backward" => ArchiveDirection::Backward,
            _ => {
                error!("Invalid direction: {}", progress.direction);
                return;
            }
        };

        let ck_record = ArchiveProgress {
            exchange: progress.exchange.clone(),
            symbol: progress.symbol.clone(),
            period: progress.period.clone(),
            direction: direction_enum,
            last_archived_time: progress.last_archived_time,
            completed: progress.completed,
            updated_at: progress.updated_at,
        };

        if let Err(err) = get_ck_db().insert_batch(&[ck_record]).await {
            error!(?err, "Failed to update archive progress");
        } else {
            info!(?progress, "Archive progress updated");
        }
    }
}
// ========== Main Archive Logic ==========

pub async fn run_archive_task(task: ArchiveTask) -> Result<()> {
    let tf_str = task.tf.to_str();
    let tf_ms = task.tf.to_period(); // 每根K线的毫秒数

    // === 起始时间逻辑 ===
    let mut current_time: i64 = match task.window.start_time {
        Some(start) => start,
        None => {
            let fallback_time = match task.direction {
                ArchiveDirection::Forward => Utc::now().timestamp_millis() - 86_400_000,
                ArchiveDirection::Backward => Utc::now().timestamp_millis(),
            };

            match ProgressTracker::get_progress(
                &task.symbol,
                &task.exchange,
                tf_str,
                task.direction.as_str(),
            )
                .await
            {
                Some(p) => match i64::try_from(p.last_archived_time) {
                    Ok(last_ts) => advance_time(last_ts, tf_ms, ArchiveDirection::Forward)
                        .unwrap_or(fallback_time),
                    Err(_) => fallback_time,
                },
                None => fallback_time,
            }
        }
    };

    let end_time = task
        .window
        .end_time
        .unwrap_or_else(|| Utc::now().timestamp_millis());

    let fetcher = BinanceFetcher::new();
    let writer = ClickhouseWriter::new();

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
            .await?;

        if klines.is_empty() {
            info!("No Kline data between {start} ~ {end}, ending task.");
            break;
        }

        if !is_kline_continuous(&klines, tf_ms) {
            warn!("Kline gap detected between {start} ~ {end}");
            // 可扩展：记录、补全、跳过或失败
        }

        writer
            .write_batch(&klines, &task.exchange, &task.symbol, tf_str)
            .await?;

        // === 更新归档进度 ===
        let last_ts = klines
            .iter()
            .map(|k| k.close_time as i64)
            .max()
            .unwrap_or(current_time);


        ProgressTracker::update_progress(ArchiveProgressRecord {
            symbol: task.symbol.clone(),
            exchange: task.exchange.clone(),
            period: tf_str.to_string(),
            direction: task.direction.as_str().to_string(),
            last_archived_time: last_ts as u64,
            completed: match task.direction {
                ArchiveDirection::Forward => last_ts >= end_time,
                ArchiveDirection::Backward => last_ts <= end_time,
            } as u8,
            updated_at: Utc::now().naive_utc(),
        })
            .await;


        // === 推进到下一段时间 ===
        current_time = advance_time(last_ts, tf_ms, task.direction).unwrap_or_else(|| {
            warn!("Failed to advance current_time (overflow). Ending task.");
            end_time
        });
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
        // 请求 K线数据
        let summaries = get_futures_market()
            .klines(symbol, tf, limit, start, end)
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
///
pub async fn historical_maintenance_process(symbol: String, exchange: String, tf: TimeFrame) {
    // 构造归档任务
    let task = ArchiveTask {
        symbol,
        exchange,
        tf,
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
