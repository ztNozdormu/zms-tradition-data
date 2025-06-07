use crate::domain::model::SortOrder;
use crate::infra::external::binance::market::KlineSummary;
use base64::Engine;
use diesel::{AsChangeset, Identifiable, Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};

/// 加密货币k线数据信息表模型
#[derive(Debug, Queryable, Selectable, Serialize, Deserialize, Identifiable, Clone)]
#[diesel(table_name = crate::schema::market_kline)]
pub struct MarketKline {
    // 唯一标识符 exchange+symbol+time_frame+close_time base64编码
    pub id: String,

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
#[derive(Debug, Identifiable, Insertable, AsChangeset, Serialize, Deserialize, Clone)]
#[diesel(table_name = crate::schema::market_kline)]
pub struct NewOrUpdateMarketKline {
    // 唯一标识符 exchange+symbol+time_frame+close_time base64编码
    pub id: String,
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
            id: encode_market_kline_pk(exchange, symbol, period, s.close_time),
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
/// 生成组合主键的 Base64 表示
pub fn encode_market_kline_pk(
    exchange: &str,
    symbol: &str,
    time_frame: &str,
    close_time: i64,
) -> String {
    // 将各字段用分隔符连接
    let raw = format!("{}|{}|{}|{}", exchange, symbol, time_frame, close_time);
    // Base64 编码
    base64::encode(raw)
}

#[derive(Debug, Clone)]
pub struct MarketKlineFilter {
    pub exchange: Option<String>,
    pub symbol: Option<String>,
    pub time_frame: Option<String>,
    pub close_time: Option<i64>,
    pub sort_by_close_time: Option<SortOrder>,
    pub page: Option<usize>,
    pub page_size: Option<usize>,
}
