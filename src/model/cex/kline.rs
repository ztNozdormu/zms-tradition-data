use barter::barter_xchange::exchange::binance::model::KlineSummary;
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

impl MarketKline {
    pub fn from_summary(
        summary: KlineSummary,
        exchange: impl Into<String>,
        symbol: impl Into<String>,
        period: impl Into<String>,
    ) -> MarketKline {
        MarketKline {
            exchange: exchange.into(),
            symbol: symbol.into(),
            period: period.into(),
            open_time: summary.open_time,
            close_time: summary.close_time,
            open: summary.open,
            high: summary.high,
            low: summary.low,
            close: summary.close,
            volume: summary.volume,
            quote_asset_volume: summary.quote_asset_volume,
            number_of_trades: summary.number_of_trades as u64,
            taker_buy_base_asset_volume: summary.taker_buy_base_asset_volume,
            taker_buy_quote_asset_volume: summary.taker_buy_quote_asset_volume,
        }
    }
}
