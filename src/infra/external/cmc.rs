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

pub struct CmcSigner;
impl BuildStrategy for CmcSigner {
    fn build<Request>(&self, _request: Request, builder: RequestBuilder) -> Result<reqwest::Request, SocketError>
    where
        Request: RestRequest
    {
        builder.header("X-CMC_PRO_API_KEY", must_get_env("COIN_MARKET_KEY").as_str()).build().map_err(SocketError::from)
    }
}

// Configuration required to sign every CoinMarketCap `RestRequest`
// struct CmcSignConfig<'a> {
//     api_key: &'a str,
//     time: DateTime<Utc>,
//     method: reqwest::Method,
//     path: Cow<'static, str>,
// }
//
// impl Signer for CmcSigner {
//     type Config<'a>
//     = CmcSignConfig<'a>
//     where
//         Self: 'a;
//
//     fn config<'a, Request>(
//         &'a self,
//         request: Request,
//         _: &RequestBuilder,
//     ) -> Result<Self::Config<'a>, SocketError>
//     where
//         Request: RestRequest,
//     {
//         Ok(CmcSignConfig {
//             api_key: must_get_env("COIN_MARKET_KEY").as_str(),
//             time: Utc::now(),
//             method: Request::method(),
//             path: request.path(),
//         })
//     }
//
//     fn add_bytes_to_sign<M>(_mac: &mut M, _config: &Self::Config<'_>)
//     where
//         M: Mac,
//     {
//
//     }
//
//     fn build_signed_request(
//         config: Self::Config<'_>,
//         builder: RequestBuilder,
//         _signature: String,
//     ) -> Result<Request, SocketError> {
//         // Add CoinMarketCap required Headers & build reqwest::Request
//         builder
//             .header("X-CMC_PRO_API_KEY", config.api_key)
//             .build()
//             .map_err(SocketError::from)
//     }
// }


/// Parser for CoinMarketCap responses
pub struct CmcParser;

impl HttpParser for CmcParser {
    type ApiError = serde_json::Value;
    type OutputError = ExecutionError;

    fn parse_api_error(&self, status: StatusCode, api_error: Self::ApiError) -> Self::OutputError {
        let error = api_error.to_string();
        ExecutionError::Socket(SocketError::HttpResponse(status, error))
    }
}

#[derive(Debug, Error)]
pub enum ExecutionError {
    #[error("SocketError: {0}")]
    Socket(#[from] SocketError),
}

#[derive(Debug, Deserialize)]
pub struct CmcStatus {
    pub timestamp: String,
    pub error_code: i32,
    pub error_message: Option<String>,
}

pub struct CoinMarketCap;

impl CoinMarketCap {
    pub fn new () -> Self { CoinMarketCap }
    pub fn get_coin_latest(){
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_coin_latest() {
        CoinMarketCap::get_coin_latest();
    }
}
