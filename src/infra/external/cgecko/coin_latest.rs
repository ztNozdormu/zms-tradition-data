use barter::barter_integration::protocol::http::rest::RestRequest;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use barter::barter_integration;
use barter::barter_integration::error::SocketError;
use barter::barter_integration::protocol::http::HttpParser;
use barter::barter_integration::protocol::http::private::encoder::HexEncoder;
use barter::barter_integration::protocol::http::private::RequestSigner;
use barter::barter_integration::protocol::http::rest::client::RestClient;
use reqwest::StatusCode;
use serde::de::DeserializeOwned;
use tracing::{error, info};
use crate::infra::external::cgecko::{CgeckoParser, CgeckoSigner, CgeckoStatus, ExecutionError};
use crate::infra::external::cgecko::constant::{BASE_URL, COIN_LATEST};

pub struct FetchCoinRequest(CoinQueryParams);


#[derive(Debug, Serialize, Default)]
pub struct CoinQueryParams {
    pub vs_currency: String,                          // required
    pub ids: Option<String>,                          // comma-separated
    pub names: Option<String>,                        // comma-separated
    pub symbols: Option<String>,                      // comma-separated
    pub include_tokens: Option<String>,               // "all" or "top"
    pub category: Option<String>,                     // category filter
    pub order: Option<String>,                        // e.g., "market_cap_desc"
    pub per_page: Option<u32>,                        // 1 ~ 250
    pub page: Option<u32>,                            // pagination
    pub sparkline: Option<bool>,                      // include sparkline
    pub price_change_percentage: Option<String>,      // e.g., "1h,24h,7d"
    pub locale: Option<String>,                       // e.g., "en"
    pub precision: Option<String>,                    // decimal precision
}

impl RestRequest for FetchCoinRequest {
    type Response = CoinResponse;
    type QueryParams = CoinQueryParams;
    type Body = ();

    fn path(&self) -> Cow<'static, str> {
        Cow::Borrowed(COIN_LATEST)
    }

    fn method() -> reqwest::Method {
        reqwest::Method::GET
    }

    fn query_params(&self) -> Option<&Self::QueryParams> {
        Some(&self.0)
    }

}

#[derive(Debug, Deserialize)]
pub struct CoinResponse (pub Vec<CoinListInfo>);



#[derive(Debug, Serialize, Deserialize)]
pub struct CoinListInfo {
    pub id: String,
    pub symbol: String,
    pub name: String,
    pub image: String,
    pub current_price: f64,
    pub market_cap: f64,
    pub market_cap_rank: u32,
    pub fully_diluted_valuation: Option<f64>,
    pub total_volume: f64,
    pub high_24h: f64,
    pub low_24h: f64,
    pub price_change_24h: f64,
    pub price_change_percentage_24h: f64,
    pub market_cap_change_24h: f64,
    pub market_cap_change_percentage_24h: f64,
    pub circulating_supply: f64,
    pub total_supply: Option<f64>,
    pub max_supply: Option<f64>,
    pub ath: f64,
    pub ath_change_percentage: f64,
    pub ath_date: String, // can also use chrono::DateTime<FixedOffset> if parsing
    pub atl: f64,
    pub atl_change_percentage: f64,
    pub atl_date: String,
    pub roi: Option<serde_json::Value>, // Can replace with typed struct if known
    pub last_updated: String,
}


pub async fn get_coin_latest() {

    // Build Ftx configured RequestSigner for signing http requests with hex encoding
    let request_signer = CgeckoSigner;

    // Build RestClient with Ftx configuration
    let rest_client = RestClient::new(BASE_URL, request_signer, CgeckoParser);

    // FetchCoinRequest
    let fetch_request: FetchCoinRequest = FetchCoinRequest(CoinQueryParams {
        vs_currency: "usd".to_string(),
        ids: None,
        order: Some("market_cap_desc".to_string()),
        per_page: Some(250),
        page: Some(1),
        sparkline: Some(false),
        price_change_percentage: Some("1h,24h,7d".to_string()),
        ..Default::default()
    });

    // Fetch Result<FetchBalancesResponse, ExecutionError>
    let response: Result<
        (CoinResponse, barter_integration::metric::Metric),
        ExecutionError,
    > = rest_client.execute(fetch_request).await;

    if let Ok((coin_response, _)) = response {
        for coin in &coin_response.0 {
            info!(
                "{} ({}) - Price: ${}, 24h: {:.2}%",
                coin.name,
                coin.symbol,
                coin.current_price,
                coin.market_cap,
            );
        }
    } else {
        error!("Failed to fetch coin data.");
    }
}