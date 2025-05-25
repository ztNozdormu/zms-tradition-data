use crate::domain::model::SortOrder;
use barter::barter_xchange::exchange::binance::model::KlineSummary;
use bigdecimal::BigDecimal;
use chrono::{NaiveDate, NaiveDateTime};
use diesel::{AsChangeset, Identifiable, Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};

/// 加密货币k线数据信息表模型
#[derive(Debug, Queryable, Selectable, Serialize, Deserialize, Identifiable, Clone)]
#[diesel(table_name = crate::schema::market_kline)]
#[diesel(primary_key(exchange, symbol, time_frame, close_time))]
pub struct MarketKline {
    /// 交易所名称，例如 binance
    pub exchange: String,

    /// 交易对，例如 BTCUSDT
    pub symbol: String,

    /// K线周期，例如 1m、5m、1h
    pub time_frame: String,

    /// K线起始时间戳（毫秒）
    pub open_time: i64,

    /// 开盘价
    pub open: f64,

    /// 最高价
    pub high: f64,

    /// 最低价
    pub low: f64,

    /// 收盘价
    pub close: f64,

    /// 成交量（基础资产）
    pub volume: f64,

    /// K线结束时间戳（毫秒）——主键的一部分
    pub close_time: i64,

    /// 成交量（计价资产）
    pub quote_asset_volume: Option<f64>,

    /// 成交笔数
    pub number_of_trades: Option<u64>,

    /// 买方成交量（基础资产）
    pub taker_buy_base_asset_volume: Option<f64>,

    /// 买方成交量（计价资产）
    pub taker_buy_quote_asset_volume: Option<f64>,
}

/// 用于创建新加密货币详细信息的模型
#[derive(Debug, Insertable, AsChangeset, Serialize, Deserialize, Clone)]
#[diesel(table_name = crate::schema::market_kline)]
#[diesel(primary_key(exchange, symbol, time_frame, close_time))]
pub struct NewOrUpdateMarketKline {
    /// 交易所名称，例如 binance
    pub exchange: String,

    /// 交易对，例如 BTCUSDT
    pub symbol: String,

    /// K线周期，例如 1m、5m、1h
    pub time_frame: String,

    /// K线起始时间戳（毫秒）
    pub open_time: i64,

    /// 开盘价
    pub open: f64,

    /// 最高价
    pub high: f64,

    /// 最低价
    pub low: f64,

    /// 收盘价
    pub close: f64,

    /// 成交量（基础资产）
    pub volume: f64,

    /// K线结束时间戳（毫秒）——主键的一部分
    pub close_time: i64,

    /// 成交量（计价资产）
    pub quote_asset_volume: Option<f64>,

    /// 成交笔数
    pub number_of_trades: Option<u64>,

    /// 买方成交量（基础资产）
    pub taker_buy_base_asset_volume: Option<f64>,

    /// 买方成交量（计价资产）
    pub taker_buy_quote_asset_volume: Option<f64>,
}

// 实现从 KlineSummary 到 NewOrUpdateCoinDataInfo 的转换
impl From<(&KlineSummary, &str, &str, &str)> for NewOrUpdateMarketKline {
    fn from((s, exchange, symbol, period): (&KlineSummary, &str, &str, &str)) -> Self {
        NewOrUpdateMarketKline {
            exchange: exchange.to_string(),
            symbol: symbol.to_string(),
            time_frame: period.to_string(),

            open_time: s.open_time,
            open: s.open,
            high: s.high,
            low: s.low,
            close: s.close,
            volume: s.volume,
            close_time: s.close_time,

            quote_asset_volume: Some(s.quote_asset_volume),
            number_of_trades: Some(s.number_of_trades as u64),
            taker_buy_base_asset_volume: Some(s.taker_buy_base_asset_volume),
            taker_buy_quote_asset_volume: Some(s.taker_buy_quote_asset_volume),
        }
    }
}

#[derive(Debug, Clone)]
pub struct MarketKlineFilter {
    pub exchange: Option<String>,
    pub symbol: Option<String>,
    pub time_frame: Option<String>,
    pub close_time: Option<i64>,
    pub sort_by_rank: Option<SortOrder>,
    pub page: Option<usize>,
    pub page_size: Option<usize>,
}
