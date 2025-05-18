/// CoinMarketCap API. All requests should target domain
pub const BASE_URL: &str = "https://api.coingecko.com";

/// https://docs.coingecko.com/v3.0.1/reference/coins-markets
/// This endpoint allows you to query all the supported coins with price, market cap, volume and market related data
pub const COIN_LATEST: &str = "/api/v3/coins/markets";

/// https://api.coingecko.com/api/v3/coins/{id}
/// This endpoint allows you to query all the metadata (image, websites, socials, description, contract address, etc.)
/// and market data (price, ATH, exchange tickers, etc.) of a coin from the CoinGecko coin page based on a particular coin ID
pub const COIN_DATA: &str = "/api/v3/coins";
