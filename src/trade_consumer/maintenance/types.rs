use crate::model::TimeFrame;
use crate::model::cex::kline::MarketKline;
use barter::barter_xchange::exchange::binance::model::KlineSummary;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::sync::Arc;

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
    pub window: Vec<ArchiveWindow>,
    pub direction: ArchiveDirection,
}

#[derive(Debug, Clone)]
pub struct ArchiveWindow {
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
}

/// convert KlineSummary to MarketKline
impl From<(&KlineSummary, &str, &str, &str)> for MarketKline {
    fn from((s, exchange, symbol, period): (&KlineSummary, &str, &str, &str)) -> Self {
        MarketKline {
            exchange: exchange.to_string(),
            symbol: symbol.to_string(),
            period: period.to_string(),

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

impl TimeFrame {
    // backtrack_count 默认回溯近五年数据即可
    pub fn backtrack_count(&self) -> usize {
        let base = 1000;
        match self {
            TimeFrame::M1 => base * 3,
            TimeFrame::M3 => base * 3,
            TimeFrame::M5 => base * 5,
            TimeFrame::M15 => base * 5,
            TimeFrame::M30 => base * 5,
            TimeFrame::H1 => base * 5,
            TimeFrame::H2 => base * 5,
            TimeFrame::H4 => base * 5,
            TimeFrame::H6 => base,
            TimeFrame::H8 => base,
            TimeFrame::H12 => base,
            TimeFrame::D1 => 700,
            TimeFrame::D3 => 700,
            TimeFrame::W1 => 240,
            TimeFrame::M1L => 60,
        }
    }
}
