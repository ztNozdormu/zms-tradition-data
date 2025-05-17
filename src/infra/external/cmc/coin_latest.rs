use crate::infra::external::cmc::constant::{BASE_URL, COIN_LATEST};
use crate::infra::external::cmc::{CmcParser, CmcSigner, CmcStatus, ExecutionError};
use barter::barter_integration;
use barter::barter_integration::protocol::http::private::RequestSigner;
use barter::barter_integration::protocol::http::private::encoder::HexEncoder;
use barter::barter_integration::protocol::http::rest::RestRequest;
use barter::barter_integration::protocol::http::rest::client::RestClient;
use serde::Deserialize;
use std::borrow::Cow;
use tracing::info;

/// Request for CMC /v1/cryptocurrency/map
pub struct FetchCoinMapRequest;

impl RestRequest for FetchCoinMapRequest {
    type Response = CoinMapResponse;
    type QueryParams = ();
    type Body = ();

    fn path(&self) -> Cow<'static, str> {
        Cow::Borrowed(COIN_LATEST)
    }

    fn method() -> reqwest::Method {
        reqwest::Method::GET
    }
}

#[derive(Debug, Deserialize)]
pub struct CoinMapResponse {
    pub status: CmcStatus,
    pub data: Vec<CoinInfo>,
}

#[derive(Debug, Deserialize)]
pub struct CoinInfo {
    pub id: u64,
    pub name: String,
    pub symbol: String,
    pub slug: String,
    pub is_active: i32,
    pub rank: Option<u32>,
    pub platform: Option<serde_json::Value>,
}

async fn get_coin_latest() {
    // Build Ftx configured RequestSigner for signing http requests with hex encoding
    let request_signer = CmcSigner;

    // Build RestClient with Ftx configuration
    let rest_client = RestClient::new(BASE_URL, request_signer, CmcParser);

    // Fetch Result<FetchBalancesResponse, ExecutionError>
    let response: Result<(CoinMapResponse, barter_integration::metric::Metric), ExecutionError> =
        rest_client.execute(FetchCoinMapRequest).await;

    info!("response: {:}", response.is_ok());
}
