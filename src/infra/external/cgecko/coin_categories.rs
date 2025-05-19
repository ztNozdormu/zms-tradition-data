use crate::infra::external::cgecko::constant::COIN_CATEGORIES;
use barter::barter_integration::protocol::http::rest::RestRequest;
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
    pub id: String,                          // category ID
    pub name: String,                        // category name
    pub market_cap: Option<f64>,             // category market cap
    pub market_cap_change_24h: Option<f64>,  // market cap change in 24h
    pub content: Option<String>,             // description, nullable
    pub top_3_coins_id: Option<Vec<String>>, // coin IDs (if available)
    pub top_3_coins: Option<Vec<String>>,    // image URLs of top 3 coins
    pub volume_24h: Option<f64>,             // 24h volume
    pub updated_at: Option<String>,          // last update time
}
