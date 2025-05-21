mod coin_category;
mod coin_data_info;
mod coin_rank_info;



#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::log_utils::{fmt_bigdecimal, fmt_naive_date};
    use crate::common::utils::format_opt_decimal;
    use crate::log_fields;
    use bigdecimal::BigDecimal;
    use tracing::error;
    use crate::infra::external::cgecko::DefaultCoinGecko;

    #[tokio::test]
    async fn test_get_coin_rank() {
        listen_tracing::setup_tracing();
        let dcg = DefaultCoinGecko::default();
        let conin_list = dcg.get_coin_latest().await;
        for coin in &conin_list {
            log_fields!(info,
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
                log_fields!(info,
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
            log_fields!(info,
                     "id" => categorie.id,
                     "name" => categorie.name,
                     "market_cap" => format_opt_decimal(&categorie.market_cap),
                );
        }
    }
}