use crate::common::serde_fun::option_obj_to_value;
use crate::infra::external::binance::meta::Symbol;
use base64::Engine;
use diesel::{AsChangeset, Identifiable, Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};

/// 加密货币k线数据信息表模型
#[derive(Debug, Queryable, Selectable, Serialize, Deserialize, Identifiable, Clone)]
#[diesel(table_name = crate::schema::market_symbol)]
pub struct MarketSymbol {
    /// 唯一标识符 exchange+symbol base64编码
    pub id: String,

    /// 交易所，如 Binance、OKX
    pub exchange: String,

    /// 交易对，如 "ONEUSDT"
    pub symbol: String,

    /// 交易对名称，通常与 symbol 一致
    pub pair: String,

    /// 合约类型，如 "PERPETUAL"
    pub contract_type: String,

    /// 交割日期（毫秒时间戳）
    pub delivery_date: i64,

    /// 上线日期（毫秒时间戳）
    pub onboard_date: i64,

    /// 状态，如 "TRADING"
    pub status: String,

    /// 维持保证金百分比，字符串表示浮点数
    pub maint_margin_percent: String,

    /// 起始保证金百分比
    pub required_margin_percent: String,

    /// 基础资产，如 "ONE"
    pub base_asset: String,

    /// 报价资产，如 "USDT"
    pub quote_asset: String,

    /// 保证金资产
    pub margin_asset: String,

    /// 价格精度，如 5 表示小数点后 5 位
    pub price_precision: u64,

    /// 数量精度（最小下单单位）
    pub quantity_precision: u64,

    /// 基础资产精度
    pub base_asset_precision: u64,

    /// 报价资产精度
    pub quote_precision: u64,

    /// 合约基础类型，如 "COIN"
    pub underlying_type: String,

    /// 合约子类型，例如 ["Layer-2"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub underlying_sub_type: Option<serde_json::Value>,

    /// 触发保护百分比
    pub trigger_protect: String,

    /// 清算手续费比例
    pub liquidation_fee: String,

    /// 市价吃单最大偏移保护
    pub market_take_bound: String,

    /// 最大移动下单限制
    pub max_move_order_limit: u64,

    /// 过滤器列表（结构不固定）
    pub filters: Option<serde_json::Value>,

    /// 支持的下单类型，如 ["LIMIT", "MARKET"]
    pub order_types: Option<serde_json::Value>,

    /// 支持的 TIF 策略，如 ["GTC", "IOC"]
    pub time_in_force: Option<serde_json::Value>,

    /// 权限集，如 ["GRID", "COPY"]
    pub permission_sets: Option<serde_json::Value>,
}

/// 用于创建新加密货币详细信息的模型
#[derive(Debug, Identifiable, Insertable, AsChangeset, Serialize, Deserialize, Clone)]
#[diesel(table_name = crate::schema::market_symbol)]
pub struct NewOrUpdateMarketSymbol {
    /// 唯一标识符 exchange+symbol base64编码
    pub id: String,

    /// 交易所，如 Binance、OKX
    pub exchange: String,

    /// 交易对，如 "ONEUSDT"
    pub symbol: String,

    /// 交易对名称，通常与 symbol 一致
    pub pair: String,

    /// 合约类型，如 "PERPETUAL"
    pub contract_type: String,

    /// 交割日期（毫秒时间戳）
    pub delivery_date: i64,

    /// 上线日期（毫秒时间戳）
    pub onboard_date: i64,

    /// 状态，如 "TRADING"
    pub status: String,

    /// 维持保证金百分比，字符串表示浮点数
    pub maint_margin_percent: String,

    /// 起始保证金百分比
    pub required_margin_percent: String,

    /// 基础资产，如 "ONE"
    pub base_asset: String,

    /// 报价资产，如 "USDT"
    pub quote_asset: String,

    /// 保证金资产
    pub margin_asset: String,

    /// 价格精度，如 5 表示小数点后 5 位
    pub price_precision: u64,

    /// 数量精度（最小下单单位）
    pub quantity_precision: u64,

    /// 基础资产精度
    pub base_asset_precision: u64,

    /// 报价资产精度
    pub quote_precision: u64,

    /// 合约基础类型，如 "COIN"
    pub underlying_type: String,

    /// 合约子类型，例如 ["Layer-2"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub underlying_sub_type: Option<serde_json::Value>,

    /// 触发保护百分比
    pub trigger_protect: String,

    /// 清算手续费比例
    pub liquidation_fee: String,

    /// 市价吃单最大偏移保护
    pub market_take_bound: String,

    /// 最大移动下单限制
    pub max_move_order_limit: u64,

    /// 过滤器列表（结构不固定）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<serde_json::Value>,

    /// 支持的下单类型，如 ["LIMIT", "MARKET"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_types: Option<serde_json::Value>,

    /// 支持的 TIF 策略，如 ["GTC", "IOC"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_in_force: Option<serde_json::Value>,

    /// 权限集，如 ["GRID", "COPY"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission_sets: Option<serde_json::Value>,
}

// 实现从 Symbol 到 NewOrUpdateMarketSymbol 的转换
impl From<Symbol> for NewOrUpdateMarketSymbol {
    fn from(s: Symbol) -> Self {
        NewOrUpdateMarketSymbol {
            id: encode_market_kline_pk("binance", &s.symbol),
            exchange: "binance".to_string(),
            symbol: s.symbol,

            pair: s.pair,
            contract_type: s.contract_type,
            delivery_date: s.delivery_date,
            onboard_date: s.onboard_date,
            status: s.status,
            maint_margin_percent: s.maint_margin_percent,
            required_margin_percent: s.required_margin_percent,
            base_asset: s.base_asset,
            base_asset_precision: s.base_asset_precision,
            quote_asset: s.quote_asset,
            margin_asset: s.margin_asset,
            price_precision: s.price_precision,
            quote_precision: s.quote_precision,
            underlying_type: s.underlying_type,
            underlying_sub_type: option_obj_to_value(Some(s.underlying_sub_type)),
            trigger_protect: s.trigger_protect,
            liquidation_fee: s.liquidation_fee,
            market_take_bound: s.market_take_bound,
            order_types: option_obj_to_value(Some(s.order_types)),
            time_in_force: option_obj_to_value(s.time_in_force),
            filters: option_obj_to_value(Some(s.filters)),
            quantity_precision: s.quantity_precision,
            max_move_order_limit: s.max_move_order_limit,
            permission_sets: option_obj_to_value(Some(s.permission_sets)),
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
