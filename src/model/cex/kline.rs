use clickhouse::Row;
use serde::Serialize;

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
