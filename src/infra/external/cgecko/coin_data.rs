use crate::common::serde_fun;
use crate::infra::external::cgecko::constant::COIN_DATA;
use barter::barter_integration::protocol::http::rest::RestRequest;
use bigdecimal::BigDecimal;
use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::collections::HashMap;

pub struct FetchCoinDataRequest {
    pub coin_id: String,
    pub query_params: CoinDataQueryParams,
}

#[derive(Debug, Clone, Serialize)]
pub struct CoinDataQueryParams {
    /// Include all localized languages in the response
    pub localization: bool,

    /// Include tickers data
    pub tickers: bool,

    /// Include market data
    pub market_data: bool,

    /// Include community data
    pub community_data: bool,

    /// Include developer data
    pub developer_data: bool,

    /// Include sparkline 7 days data
    pub sparkline: bool,
}

impl Default for CoinDataQueryParams {
    fn default() -> Self {
        CoinDataQueryParams {
            localization: false,
            tickers: false,
            market_data: false,
            community_data: false,
            developer_data: false,
            sparkline: false,
        }
    }
}

impl RestRequest for FetchCoinDataRequest {
    type Response = CoinDataResponse;
    type QueryParams = CoinDataQueryParams;
    type Body = ();

    fn path(&self) -> Cow<'static, str> {
        // let url = format!("{}/{}", COIN_DATA, self.coin_id).as_str();
        // Cow::Borrowed(url)
        Cow::Owned(format!("{}/{}", COIN_DATA, self.coin_id))
    }

    fn method() -> reqwest::Method {
        reqwest::Method::GET
    }

    fn query_params(&self) -> Option<&Self::QueryParams> {
        Some(&self.query_params)
    }
}

#[derive(Debug, Deserialize)]
pub struct CoinDataResponse(pub CoinData);

#[derive(Debug, Deserialize, Serialize)]
pub struct CoinData {
    pub id: String,
    pub symbol: String,
    pub name: String,
    pub web_slug: Option<String>,
    pub asset_platform_id: Option<String>,
    pub platforms: Option<HashMap<String, String>>,
    pub block_time_in_minutes: Option<u32>,
    pub hashing_algorithm: Option<String>,
    pub categories: Option<Vec<String>>,
    pub preview_listing: Option<bool>,
    pub public_notice: Option<String>,
    pub additional_notices: Option<Vec<String>>,
    pub description: Option<HashMap<String, String>>,
    pub country_origin: Option<String>,
    // #[serde(deserialize_with = "serde_fun::deserialize_datetime_option")]
    pub genesis_date: Option<NaiveDate>,
    #[serde(deserialize_with = "serde_fun::deserialize_option_string2bigdcimal")]
    pub sentiment_votes_up_percentage: Option<BigDecimal>,
    #[serde(deserialize_with = "serde_fun::deserialize_option_string2bigdcimal")]
    pub sentiment_votes_down_percentage: Option<BigDecimal>,
    pub watchlist_portfolio_users: Option<u32>,
    pub market_cap_rank: Option<u32>,
    #[serde(deserialize_with = "serde_fun::deserialize_datetime_option")]
    pub last_updated: Option<NaiveDateTime>,
}
