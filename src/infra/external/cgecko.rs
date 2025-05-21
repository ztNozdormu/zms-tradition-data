pub mod coin_categories;
pub mod coin_data;
pub mod coin_rank;
mod constant;

use crate::infra::external::cgecko::coin_categories::{CoinCategories, FetchCoinCategoriesRequest};
use crate::infra::external::cgecko::coin_data::{
    CoinData, CoinDataQueryParams, FetchCoinDataRequest,
};
/// https://docs.coingecko.com/v3.0.1/reference/introduction
/// CoinGecko API key signer using header injection only (no HMAC)
use crate::infra::external::cgecko::coin_rank::{
    CoinQueryParams, CoinRank, CoinResponse, FetchCoinRequest,
};
use crate::infra::external::cgecko::constant::BASE_URL;
use crate::infra::external::{CommonExternalParser, ExecutionError};
use barter::barter_integration;
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

    pub async fn get_coin_rank(&self) -> Vec<CoinRank> {
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
            Ok((response, _)) => response.0,
            Err(err) => {
                error!("Failed to fetch coin data: {:?}", err);
                Vec::new()
            }
        }
    }

    pub async fn get_coin_data(&self, coin_id: &str) -> Option<CoinData> {
        let fetch_request = FetchCoinDataRequest {
            coin_id: coin_id.to_string(),
            query_params: CoinDataQueryParams::default(),
        };

        match self.rest_client.execute(fetch_request).await {
            Ok((response, _)) => Some(response.0),
            Err(err) => {
                error!("Failed to fetch coin data: {:?}", err);
                None
            }
        }
    }

    pub async fn get_categories(&self) -> Vec<CoinCategories> {
        let fetch_request = FetchCoinCategoriesRequest;

        match self.rest_client.execute(fetch_request).await {
            Ok((response, _)) => response.0,
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
    use crate::common::log_utils::{fmt_bigdecimal, fmt_naive_date};
    use crate::common::utils::format_opt_decimal;
    use crate::trace_fields;
    use bigdecimal::BigDecimal;

    #[tokio::test]
    async fn test_get_coin_rank() {
        listen_tracing::setup_tracing();
        let dcg = DefaultCoinGecko::default();
        let conin_list = dcg.get_coin_rank().await;
        for coin in &conin_list {
            trace_fields!(info,
                 "id" => coin.id,
                 "name" => coin.name,
                 "symbol" => coin.symbol,
                 "current_price" => format_opt_decimal(&coin.current_price),
                 "market_cap" => format_opt_decimal(&coin.market_cap),
            );
        }
    }

    #[tokio::test]
    async fn test_get_coin_data() {
        listen_tracing::setup_tracing();
        let dcg = DefaultCoinGecko::default();
        let coin_id = "bitcoin";
        let coin_data = dcg.get_coin_data(coin_id).await;
        match coin_data {
            Some(coin_data) => {
                trace_fields!(info,
                     "id" => coin_data.id,
                     "name" => coin_data.name,
                     "symbol" => coin_data.symbol,
                     "categories len" => coin_data.categories.unwrap_or(Vec::new()).len(),
                     "market_cap_rank" => fmt_bigdecimal(&coin_data.sentiment_votes_up_percentage),
                     "genesis_date" => fmt_naive_date(&coin_data.genesis_date),
                );
            }
            None => {
                error!("Failed to fetch coin data");
            }
        }
    }

    #[tokio::test]
    async fn test_get_categories() {
        listen_tracing::setup_tracing();
        let dcg = DefaultCoinGecko::default();
        let categories = dcg.get_categories().await;
        for categorie in &categories {
            trace_fields!(info,
                 "id" => categorie.id,
                 "name" => categorie.name,
                 "market_cap" => format_opt_decimal(&categorie.market_cap),
            );
        }
    }
}
