use crate::common::serde_fun::option_obj_to_value;
use crate::domain::model::SortOrder;
use barter::barter_xchange::exchange::binance::model::{KlineSummary, Symbol};
use base64::Engine;
use diesel::{AsChangeset, Identifiable, Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};

/// 加密货币k线数据信息表模型
#[derive(Debug, Queryable, Selectable, Serialize, Deserialize, Identifiable, Clone)]
#[diesel(table_name = crate::schema::market_symbol)]
pub struct MarketSymbol {
    /// 主键 ID，可使用交易对和交易所拼接生成（如 "binance:BTCUSDT"）
    pub id: String,

    /// 交易所，如 Binance、OKX
    pub exchange: String,

    /// 交易对名称，例如 BTCUSDT
    pub symbol: String,

    pub status: String,
    pub base_asset: String,
    pub base_asset_precision: u64,
    pub quote_asset: String,
    pub quote_precision: u64,
    pub order_types: Option<serde_json::Value>,
    pub iceberg_allowed: Option<bool>,
    pub is_spot_trading_allowed: Option<bool>,
    pub is_margin_trading_allowed: Option<bool>,
    pub filters: Option<serde_json::Value>,
}

/// 用于创建新加密货币详细信息的模型
#[derive(Debug, Identifiable, Insertable, AsChangeset, Serialize, Deserialize, Clone)]
#[diesel(table_name = crate::schema::market_symbol)]
pub struct NewOrUpdateMarketSymbol {
    // 唯一标识符 exchange+symbol base64编码
    pub id: String,
    /// 交易所名称，例如 binance
    /// 交易所，如 Binance、OKX
    pub exchange: String,

    /// 交易对名称，例如 BTCUSDT
    pub symbol: String,

    pub status: String,
    pub base_asset: String,
    pub base_asset_precision: u64,
    pub quote_asset: String,
    pub quote_precision: u64,
    pub order_types: Option<serde_json::Value>,
    pub iceberg_allowed: Option<bool>,
    pub is_spot_trading_allowed: Option<bool>,
    pub is_margin_trading_allowed: Option<bool>,
    pub filters: Option<serde_json::Value>,
}

// 实现从 Symbol 到 NewOrUpdateMarketSymbol 的转换
impl From<Symbol> for NewOrUpdateMarketSymbol {
    fn from(s: Symbol) -> Self {
        NewOrUpdateMarketSymbol {
            id: encode_market_kline_pk("binance", &s.symbol),
            exchange: "binance".to_string(),
            symbol: s.symbol,

            status: s.status,
            base_asset: s.base_asset,
            base_asset_precision: s.base_asset_precision,
            quote_asset: s.quote_asset,
            quote_precision: s.quote_precision,
            order_types: option_obj_to_value(Some(s.order_types)),
            iceberg_allowed: s.iceberg_allowed,
            is_spot_trading_allowed: s.is_spot_trading_allowed,
            is_margin_trading_allowed: s.is_margin_trading_allowed,
            filters: option_obj_to_value(Some(s.filters)),
        }
    }
}
/// 生成组合主键的 Base64 表示
pub fn encode_market_kline_pk(exchange: &str, symbol: &str) -> String {
    // 将各字段用分隔符连接
    let raw = format!("{}|{}", exchange, symbol);
    // Base64 编码
    base64::encode(raw)
}

#[derive(Debug, Clone)]
pub struct MarketSymbolFilter {
    pub exchange: Option<String>,
    pub symbol: Option<String>,
    pub page: Option<usize>,
    pub page_size: Option<usize>,
}
