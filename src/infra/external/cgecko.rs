mod coin_latest;
mod constant;


/// CoinMarketCap API key signer using header injection only (no HMAC)

use barter::barter_integration::error::SocketError;
use barter::barter_integration::protocol::http::private::Signer;
use barter::barter_integration::protocol::http::rest::RestRequest;
use barter::barter_integration::protocol::http::{BuildStrategy, HttpParser};
use hmac::Mac;
use reqwest::{RequestBuilder, StatusCode};
use serde::Deserialize;
use thiserror::Error;
use crate::common::utils::must_get_env;

pub struct CgeckoSigner;
impl BuildStrategy for CgeckoSigner {
    fn build<Request>(&self, _request: Request, builder: RequestBuilder) -> Result<reqwest::Request, SocketError>
    where
        Request: RestRequest
    {
        builder.header("X-CMC_PRO_API_KEY", must_get_env("COIN_GECKO_KEY").as_str()).build().map_err(SocketError::from)
    }
}

/// Parser for CoinMarketCap responses
pub struct CgeckoParser;

impl HttpParser for CgeckoParser {
    type ApiError = serde_json::Value;
    type OutputError = crate::infra::external::cmc::ExecutionError;

    fn parse_api_error(&self, status: StatusCode, api_error: Self::ApiError) -> Self::OutputError {
        let error = api_error.to_string();
        crate::infra::external::cmc::ExecutionError::Socket(SocketError::HttpResponse(status, error))
    }
}

#[derive(Debug, Error)]
pub enum ExecutionError {
    #[error("SocketError: {0}")]
    Socket(#[from] SocketError),
}

#[derive(Debug, Deserialize)]
pub struct CgeckoStatus {
    pub timestamp: String,
    pub error_code: i32,
    pub error_message: Option<String>,
}

pub struct CoinGecko;

impl CoinGecko {
    pub fn new () -> Self { CoinGecko }
    pub fn get_coin_latest(){
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_coin_latest() {
        CoinGecko::get_coin_latest();
    }
}
