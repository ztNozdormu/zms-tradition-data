use crate::infra::external::binance::constant;
use barter::barter_integration::protocol::http::rest::RestRequest;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// Binance 交易所返回的交易所基本信息结构
#[derive(Debug, Serialize, Deserialize)]
pub struct BinanceExchangeInfo {
    /// 时区，例如 "UTC"
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,

    /// 服务器时间戳（通常为字符串类型，也可能为数字）
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "serverTime"
    )]
    pub server_time: Option<String>,

    /// 所有支持的交易对信息
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub symbols: Option<Vec<Symbol>>,

    /// 限速策略列表（如请求次数限制等）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rate_limits: Option<Vec<RateLimit>>,

    /// 全局性过滤器列表
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exchange_filters: Option<Vec<String>>,

    /// 当前账户支持的权限，例如 ["SPOT"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<String>>,
}

/// 表示 Binance 中的一个交易对（symbol）信息
#[derive(Debug, Serialize, Deserialize)]
pub struct Symbol {
    /// 基础资产，例如 "BTC"
    #[serde(rename = "baseAsset", default, skip_serializing_if = "Option::is_none")]
    pub base_asset: Option<String>,

    /// 计价资产，例如 "USDT"
    #[serde(
        rename = "quoteAsset",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub quote_asset: Option<String>,

    /// 交易对名称，例如 "BTCUSDT"
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,

    /// 当前交易对状态，例如 "TRADING"
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,

    /// 是否支持冰山单
    #[serde(
        rename = "icebergAllowed",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub iceberg_allowed: Option<String>,

    /// 是否支持 OCO 订单
    #[serde(
        rename = "ocoAllowed",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub oco_allowed: Option<String>,

    /// 是否允许保证金交易
    #[serde(
        rename = "isMarginTradingAllowed",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_margin_trading_allowed: Option<String>,

    /// 是否允许现货交易
    #[serde(
        rename = "isSpotTradingAllowed",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_spot_trading_allowed: Option<String>,

    /// 计价资产的小数精度
    #[serde(
        rename = "quotePrecision",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub quote_precision: Option<String>,

    /// 计价资产的精确精度
    #[serde(
        rename = "quoteAssetPrecision",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub quote_asset_precision: Option<String>,

    /// 基础资产的精确精度
    #[serde(
        rename = "baseAssetPrecision",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub base_asset_precision: Option<String>,

    /// 该交易对支持的订单类型列表（如 ["LIMIT", "MARKET", ...]）
    #[serde(
        rename = "orderTypes",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub order_types: Option<Vec<String>>,

    /// 该交易对相关的交易限制信息
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Filter>>,

    /// 账户对该交易对的权限（如 ["SPOT", "MARGIN"]）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<String>>,
}

/// 表示 Binance 中的 API 访问速率限制规则
#[derive(Debug, Serialize, Deserialize)]
pub struct RateLimit {
    /// 每个 interval 内允许的最大请求数，例如 "1200"
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<String>,

    /// 时间间隔单位（如 "MINUTE", "SECOND", "DAY"）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interval: Option<String>,

    /// 每个 interval 的数量（例如 "1" 表示每 1 分钟）
    #[serde(
        rename = "intervalNum",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub interval_num: Option<String>,

    /// 限流类型（如 "REQUEST_WEIGHT", "ORDERS"）
    #[serde(
        rename = "rateLimitType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub rate_limit_type: Option<String>,
}

/// 表示 Binance 交易对中的过滤器信息（如价格限制、数量限制等）
#[derive(Debug, Serialize, Deserialize)]
pub struct Filter {
    /// 最大价格，例如 "100000.00000000"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_price: Option<String>,

    /// 过滤器类型，如 "PRICE_FILTER"、"LOT_SIZE" 等
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_type: Option<String>,

    /// 价格步长，价格精度的单位，如 "0.01000000"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tick_size: Option<String>,

    /// 最小价格
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_price: Option<String>,

    /// 最小交易数量
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_qty: Option<String>,

    /// 最大交易数量
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_qty: Option<String>,

    /// 数量步长，数量精度的单位
    #[serde(skip_serializing_if = "Option::is_none")]
    pub step_size: Option<String>,

    /// 最小名义价值（如最小成交额）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_notional: Option<String>,

    /// 最大名义价值（如最大成交额）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_notional: Option<String>,
}

pub struct FetchExchangeInfoRequest;

impl RestRequest for FetchExchangeInfoRequest {
    type Response = ExchangeInfoResponse;
    type QueryParams = ();
    type Body = ();

    fn path(&self) -> Cow<'static, str> {
        Cow::Borrowed(constant::EXCHANGE_INFO)
    }

    fn method() -> reqwest::Method {
        reqwest::Method::GET
    }
}

#[derive(Debug, Deserialize)]
pub struct ExchangeInfoResponse(pub BinanceExchangeInfo);
