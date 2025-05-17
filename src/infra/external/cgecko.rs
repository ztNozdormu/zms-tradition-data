mod coin_latest;
mod constant;

use crate::infra::external::cgecko::coin_latest::{
    CoinListInfo, CoinQueryParams, CoinResponse, FetchCoinRequest,
};
use crate::infra::external::cgecko::constant::BASE_URL;
use crate::infra::external::{CommonExternalParser, ExecutionError};
use barter::barter_integration;
/// CoinGecko API key signer using header injection only (no HMAC)
use barter::barter_integration::error::SocketError;
use barter::barter_integration::protocol::http::private::Signer;
use barter::barter_integration::protocol::http::rest::RestRequest;
use barter::barter_integration::protocol::http::rest::client::RestClient;
use barter::barter_integration::protocol::http::{BuildStrategy, HttpParser};
use hmac::Mac;
use reqwest::RequestBuilder;
use serde::Deserialize;
use std::fmt::Debug;
use tracing::{error, info};

pub struct CgeckoSigner;
impl BuildStrategy for CgeckoSigner {
    fn build<Request>(
        &self,
        _request: Request,
        builder: RequestBuilder,
    ) -> Result<reqwest::Request, SocketError>
    where
        Request: RestRequest,
    {
        // must_get_env("COIN_GECKO_KEY").as_str() CG-ZPtQ47VNptmc4zkZDU8ZTQBz todo 改为配置获取
        builder
            .header("x-cg-demo-api-key", "CG-ZPtQ47VNptmc4zkZDU8ZTQBz")
            .build()
            .map_err(SocketError::from)
    }
}

pub struct CoinGecko<'a, Strategy, Parser>
where
    Strategy: BuildStrategy,
    Parser: HttpParser,
{
    rest_client: RestClient<'a, Strategy, Parser>,
}

pub type DefaultCoinGecko<'a> = CoinGecko<'a, CgeckoSigner, CommonExternalParser>;

impl<'a> Default for DefaultCoinGecko<'a> {
    fn default() -> Self {
        Self {
            rest_client: RestClient::new(BASE_URL, CgeckoSigner, CommonExternalParser),
        }
    }
}

impl<'a, Strategy, Parser> CoinGecko<'a, Strategy, Parser>
where
    Strategy: BuildStrategy,
    Parser: HttpParser,
    <Parser as HttpParser>::OutputError: Debug,
{
    pub fn new(strategy: Strategy, parser: Parser) -> Self
    where
        Strategy: BuildStrategy,
        Parser: HttpParser,
    {
        Self {
            rest_client: RestClient::new(BASE_URL, strategy, parser),
        }
    }

    pub async fn get_coin_latest(&self) -> Vec<CoinListInfo> {
        let fetch_request = FetchCoinRequest(CoinQueryParams {
            vs_currency: "usd".to_string(),
            ids: None,
            order: Some("market_cap_desc".to_string()),
            per_page: Some(250),
            page: Some(1),
            sparkline: Some(false),
            price_change_percentage: Some("1h,24h,7d".to_string()),
            ..Default::default()
        });

        match self.rest_client.execute(fetch_request).await {
            Ok((coin_response, _)) => coin_response.0,
            Err(err) => {
                error!("Failed to fetch coin data: {:?}", err);
                Vec::new()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_coin_latest() {
        listen_tracing::setup_tracing();
        let dcg = DefaultCoinGecko::default();
        let conin_list = dcg.get_coin_latest().await;
        for coin in &conin_list {
            info!(
                "{} ({}) - Price: ${}, Market Cap: {}",
                coin.name, coin.symbol, coin.current_price, coin.market_cap,
            );
        }
    }
}
