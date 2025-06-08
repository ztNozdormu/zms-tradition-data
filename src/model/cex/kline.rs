use crate::infra::external::binance::market::KlineSummary;
use clickhouse::Row;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Row)]
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

#[derive(Debug, Deserialize, Row)]
pub struct MinMaxCloseTime {
    pub min_close_time: i64,
    pub max_close_time: i64,
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
