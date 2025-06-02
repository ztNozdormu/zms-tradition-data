use crate::infra::external::binance::constant;
use crate::infra::external::binance::meta::BinanceExchangeInfo;
use barter::barter_integration::protocol::http::rest::RestRequest;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::collections::BTreeMap;

/// 表示一根 K 线（蜡烛图）汇总数据
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct KlineSummary {
    /// K 线开盘时间（Unix 时间戳，毫秒）
    pub open_time: i64,

    /// 开盘价
    pub open: f64,

    /// 最高价
    pub high: f64,

    /// 最低价
    pub low: f64,

    /// 收盘价（该 K 线期间最后一笔成交价）
    pub close: f64,

    /// 成交量（以基础资产计）
    pub volume: f64,

    /// K 线收盘时间（Unix 时间戳，毫秒）
    pub close_time: i64,

    /// 成交额（以报价资产计）
    pub quote_asset_volume: f64,

    /// 成交笔数
    pub number_of_trades: i64,

    /// 主动买入的成交量（以基础资产计）
    pub taker_buy_base_asset_volume: f64,

    /// 主动买入的成交额（以报价资产计）
    pub taker_buy_quote_asset_volume: f64,
}

pub struct FetchKlineSummaryRequest {
    pub(crate) query_params: BTreeMap<String, String>,
}

impl RestRequest for FetchKlineSummaryRequest {
    type Response = BinanceKlineSummaryResponse;
    type QueryParams = BTreeMap<String, String>;
    type Body = ();

    fn path(&self) -> Cow<'static, str> {
        Cow::Borrowed(constant::KLINES)
    }

    fn method() -> reqwest::Method {
        reqwest::Method::GET
    }

    fn query_params(&self) -> Option<&Self::QueryParams> {
        Some(&self.query_params)
    }
}

#[derive(Debug, Deserialize)]
pub struct BinanceKlineSummaryResponse(pub Vec<KlineSummary>);
