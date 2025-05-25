// use crate::common::serde_fun;
// use crate::infra::external::cgecko::constant::COIN_LATEST;
// use barter::barter_integration::protocol::http::rest::RestRequest;
// use bigdecimal::BigDecimal;
// use chrono::NaiveDateTime;
// use serde::{Deserialize, Serialize};
// use std::borrow::Cow;
//
// pub struct FetchCoinRequest(pub CoinQueryParams);
//
// #[derive(Debug, Serialize, Default)]
// pub struct CoinQueryParams {
//     pub vs_currency: String,                     // required
//     pub ids: Option<String>,                     // comma-separated
//     pub names: Option<String>,                   // comma-separated
//     pub symbols: Option<String>,                 // comma-separated
//     pub include_tokens: Option<String>,          // "all" or "top"
//     pub category: Option<String>,                // category filter
//     pub order: Option<String>,                   // e.g., "market_cap_desc"
//     pub per_page: Option<u32>,                   // 1 ~ 250
//     pub page: Option<u32>,                       // pagination
//     pub sparkline: Option<bool>,                 // include sparkline
//     pub price_change_percentage: Option<String>, // e.g., "1h,24h,7d"
//     pub locale: Option<String>,                  // e.g., "en"
//     pub precision: Option<String>,               // decimal precision
// }
//
// impl RestRequest for FetchCoinRequest {
//     type Response = CoinResponse;
//     type QueryParams = CoinQueryParams;
//     type Body = ();
//
//     fn path(&self) -> Cow<'static, str> {
//         Cow::Borrowed(COIN_LATEST)
//     }
//
//     fn method() -> reqwest::Method {
//         reqwest::Method::GET
//     }
//
//     fn query_params(&self) -> Option<&Self::QueryParams> {
//         Some(&self.0)
//     }
// }
//
// #[derive(Debug, Deserialize)]
// pub struct CoinResponse(pub Vec<CoinRank>);
//
// #[derive(Debug, Serialize, Deserialize)]
// pub struct CoinRank {
//     pub id: String,
//     pub symbol: String,
//     pub name: String,
//     pub image: String,
//     #[serde(deserialize_with = "serde_fun::deserialize_option_string2bigdcimal")]
//     pub current_price: Option<BigDecimal>,
//     #[serde(deserialize_with = "serde_fun::deserialize_option_string2bigdcimal")]
//     pub market_cap: Option<BigDecimal>,
//     pub market_cap_rank: Option<u32>,
//     #[serde(deserialize_with = "serde_fun::deserialize_option_string2bigdcimal")]
//     pub fully_diluted_valuation: Option<BigDecimal>,
//     #[serde(deserialize_with = "serde_fun::deserialize_option_string2bigdcimal")]
//     pub total_volume: Option<BigDecimal>,
//     #[serde(deserialize_with = "serde_fun::deserialize_option_string2bigdcimal")]
//     pub high_24h: Option<BigDecimal>,
//     #[serde(deserialize_with = "serde_fun::deserialize_option_string2bigdcimal")]
//     pub low_24h: Option<BigDecimal>,
//     #[serde(deserialize_with = "serde_fun::deserialize_option_string2bigdcimal")]
//     pub price_change_24h: Option<BigDecimal>,
//     #[serde(deserialize_with = "serde_fun::deserialize_option_string2bigdcimal")]
//     pub price_change_percentage_24h: Option<BigDecimal>,
//     #[serde(deserialize_with = "serde_fun::deserialize_option_string2bigdcimal")]
//     pub market_cap_change_24h: Option<BigDecimal>,
//     #[serde(deserialize_with = "serde_fun::deserialize_option_string2bigdcimal")]
//     pub market_cap_change_percentage_24h: Option<BigDecimal>,
//     #[serde(deserialize_with = "serde_fun::deserialize_option_string2bigdcimal")]
//     pub circulating_supply: Option<BigDecimal>,
//     #[serde(deserialize_with = "serde_fun::deserialize_option_string2bigdcimal")]
//     pub total_supply: Option<BigDecimal>,
//     #[serde(deserialize_with = "serde_fun::deserialize_option_string2bigdcimal")]
//     pub max_supply: Option<BigDecimal>,
//     #[serde(deserialize_with = "serde_fun::deserialize_option_string2bigdcimal")]
//     pub ath: Option<BigDecimal>,
//     #[serde(deserialize_with = "serde_fun::deserialize_option_string2bigdcimal")]
//     pub ath_change_percentage: Option<BigDecimal>,
//     #[serde(deserialize_with = "serde_fun::deserialize_datetime_option")]
//     pub ath_date: Option<NaiveDateTime>,
//     #[serde(deserialize_with = "serde_fun::deserialize_option_string2bigdcimal")]
//     pub atl: Option<BigDecimal>,
//     #[serde(deserialize_with = "serde_fun::deserialize_option_string2bigdcimal")]
//     pub atl_change_percentage: Option<BigDecimal>,
//     #[serde(deserialize_with = "serde_fun::deserialize_datetime_option")]
//     pub atl_date: Option<NaiveDateTime>,
//     pub roi: Option<RoiInfo>, // Can replace with typed struct if known
//     #[serde(deserialize_with = "serde_fun::deserialize_datetime_option")]
//     pub last_updated: Option<NaiveDateTime>,
// }
//
// #[derive(Debug, Serialize, Deserialize)]
// pub struct RoiInfo {
//     pub times: f64,
//     pub currency: String,
//     pub percentage: f64,
// }
