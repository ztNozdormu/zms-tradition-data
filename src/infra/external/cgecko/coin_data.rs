use crate::infra::external::cgecko::constant::COIN_DATA;
use barter::barter_integration::protocol::http::rest::RestRequest;
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
pub struct CoinDataResponse(pub CoinDataInfo);

#[derive(Debug, Deserialize, Serialize)]
pub struct CoinDataInfo {
    pub id: String,
    pub symbol: String,
    pub name: String,
    pub web_slug: Option<String>,
    pub asset_platform_id: Option<String>,
    pub platforms: HashMap<String, String>,
    pub detail_platforms: HashMap<String, DetailPlatform>,
    pub block_time_in_minutes: Option<u64>,
    pub hashing_algorithm: Option<String>,
    pub categories: Vec<String>,
    pub preview_listing: bool,
    pub public_notice: Option<String>,
    pub additional_notices: Vec<String>,
    pub description: HashMap<String, String>,
    pub links: CoinLinks,
    pub image: CoinImage,
    pub country_origin: String,
    pub genesis_date: Option<String>,
    pub sentiment_votes_up_percentage: Option<f64>,
    pub sentiment_votes_down_percentage: Option<f64>,
    pub watchlist_portfolio_users: Option<u64>,
    pub market_cap_rank: Option<u32>,
    pub status_updates: Vec<serde_json::Value>, // 若需要可以进一步定义结构
    pub last_updated: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DetailPlatform {
    pub decimal_place: Option<u32>,
    pub contract_address: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CoinLinks {
    pub homepage: Vec<String>,
    pub whitepaper: Option<String>,
    #[serde(default)]
    pub blockchain_site: Vec<String>,
    pub official_forum_url: Vec<String>,
    pub chat_url: Vec<String>,
    pub announcement_url: Vec<String>,
    pub snapshot_url: Option<String>,
    pub twitter_screen_name: Option<String>,
    pub facebook_username: Option<String>,
    pub bitcointalk_thread_identifier: Option<u64>,
    pub telegram_channel_identifier: Option<String>,
    pub subreddit_url: Option<String>,
    pub repos_url: ReposUrl,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ReposUrl {
    pub github: Vec<String>,
    pub bitbucket: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CoinImage {
    pub thumb: String,
    pub small: String,
    pub large: String,
}
