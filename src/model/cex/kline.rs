use clickhouse::Row;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Row)]
pub struct MarketKline {
    pub exchange: String,
    pub symbol: String,
    pub period: String,

    pub open_time: i64,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
    pub close_time: i64,

    pub quote_asset_volume: f64,
    pub number_of_trades: u64,
    pub taker_buy_base_asset_volume: f64,
    pub taker_buy_quote_asset_volume: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Row)]
pub struct ArchiveProgress {
    pub exchange: String,
    pub symbol: String,
    pub period: String,
    pub direction: ArchiveDirection,
    pub last_archived_time: u64,
    pub completed: u8,
    pub updated_at: chrono::NaiveDateTime,
}

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
}
