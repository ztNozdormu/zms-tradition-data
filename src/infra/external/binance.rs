use crate::infra::external::CommonExternalParser;
use crate::infra::external::binance::meta::{
    BinanceExchangeInfo, FetchExchangeInfoRequest, Symbol,
};
use barter::barter_integration::error::SocketError;
use barter::barter_integration::protocol::http::private::Signer;
use barter::barter_integration::protocol::http::rest::RestRequest;
use barter::barter_integration::protocol::http::rest::client::RestClient;
use barter::barter_integration::protocol::http::{BuildStrategy, HttpParser};
use reqwest::RequestBuilder;
use std::fmt::Debug;
use tracing::error;

mod constant;
pub mod meta;

pub struct BinanceSigner;
impl BuildStrategy for BinanceSigner {
    fn build<Request>(
        &self,
        _request: Request,
        builder: RequestBuilder,
    ) -> Result<reqwest::Request, SocketError>
    where
        Request: RestRequest,
    {
        builder.build().map_err(SocketError::from)
    }
}

pub struct BinanceExchange<'a, Strategy, Parser>
where
    Strategy: BuildStrategy,
    Parser: HttpParser,
{
    rest_client: RestClient<'a, Strategy, Parser>,
}

pub type DefaultBinanceExchange<'a> = BinanceExchange<'a, BinanceSigner, CommonExternalParser>;

impl<'a> Default for DefaultBinanceExchange<'a> {
    fn default() -> Self {
        Self {
            rest_client: RestClient::new(constant::BASE_URL, BinanceSigner, CommonExternalParser),
        }
    }
}

impl<'a, Strategy, Parser> BinanceExchange<'a, Strategy, Parser>
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
            rest_client: RestClient::new(constant::BASE_URL, strategy, parser),
        }
    }

    pub async fn get_exchange_info(&self) -> Option<BinanceExchangeInfo> {
        let fetch_request = FetchExchangeInfoRequest;

        match self.rest_client.execute(fetch_request).await {
            Ok((response, _)) => Some(response.0),
            Err(err) => {
                error!("Failed to fetch coin data: {:?}", err);
                None
            }
        }
    }

    pub async fn get_symbols(&self) -> Option<Vec<Symbol>> {
        let exchange_info = self.get_exchange_info().await;

        if exchange_info.is_some() {
            exchange_info.map(|exchange_info| exchange_info.symbols)?
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::utils::format_opt_decimal;
    use bigdecimal::BigDecimal;
    use listen_tracing::trace_kv;
    use listen_tracing::tracing_utils::{fmt_bigdecimal, fmt_naive_date};
    use tracing::debug;

    #[tokio::test]
    async fn test_get_exchange_info() {
        listen_tracing::setup_tracing();
        let dbe = DefaultBinanceExchange::default();
        let exchange_info = dbe.get_exchange_info().await;
        match exchange_info {
            None => {
                debug!("Empty exchange info");
            }
            Some(exchange_info) => {
                trace_kv!(info,
                     "server_time" => exchange_info.server_time,
                     "timezone" => exchange_info.timezone,
                );
            }
        }
    }

    #[tokio::test]
    async fn test_get_symbols() {
        listen_tracing::setup_tracing();
        let dbe = DefaultBinanceExchange::default();

        let symbols = dbe.get_symbols().await;
        match symbols {
            None => {
                debug!("Empty exchange info");
            }
            Some(symbols) => {
                for symbol in &symbols {
                    trace_kv!(info,
                     "symbol" => symbol.symbol,
                     "quote_asset" => symbol.quote_asset,
                     "contract_type" => symbol.contract_type,
                    );
                }
            }
        }
    }
}
