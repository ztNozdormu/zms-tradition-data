/// CoinMarketCap API. All requests should target domain
pub const BASE_URL: &str = "https://pro-api.coinmarketcap.com";

/// https://coinmarketcap.com/api/documentation/v1/#operation/getV2CryptocurrencyInfo
///  返回历史 UTC 日期的所有加密货币的排名和排序列表。
pub const COIN_LATEST: &str = "/cryptocurrency/listings/latest";