use crate::common::serde_fun;
use crate::infra::external::cgecko::constant::COIN_CATEGORIES;
use barter::barter_integration::protocol::http::rest::RestRequest;
use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

pub struct FetchCoinCategoriesRequest;

impl RestRequest for FetchCoinCategoriesRequest {
    type Response = CoinCategoriesResponse;
    type QueryParams = ();
    type Body = ();

    fn path(&self) -> Cow<'static, str> {
        Cow::Borrowed(COIN_CATEGORIES)
    }

    fn method() -> reqwest::Method {
        reqwest::Method::GET
    }
}

#[derive(Debug, Deserialize)]
pub struct CoinCategoriesResponse(pub Vec<CoinCategories>);

#[derive(Debug, Serialize, Deserialize)]
pub struct CoinCategories {
    pub id: String,   // category ID
    pub name: String, // category
    #[serde(deserialize_with = "serde_fun::deserialize_option_string2bigdcimal")]
    pub market_cap: Option<BigDecimal>, // category market cap
    #[serde(deserialize_with = "serde_fun::deserialize_option_string2bigdcimal")]
    pub market_cap_change_24h: Option<BigDecimal>, // market cap change in 24h f64
    pub content: Option<String>, // description, nullable
    pub top_3_coins_id: Option<Vec<String>>, // coin IDs (if available)
    pub top_3_coins: Option<Vec<String>>, // image URLs of top 3 coins
    #[serde(deserialize_with = "serde_fun::deserialize_option_string2bigdcimal")]
    pub volume_24h: Option<BigDecimal>, // 24h volume
    #[serde(deserialize_with = "serde_fun::deserialize_datetime_option")]
    pub updated_at: Option<NaiveDateTime>, // last update time String
}
