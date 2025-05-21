mod coin_category;
mod coin_data_info;
mod coin_rank_info;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::VecConvert;
    use crate::common::log_utils::{fmt_bigdecimal, fmt_naive_date};
    use crate::common::utils::format_opt_decimal;
    use crate::domain::model::coin_rank_info::{CoinRankInfo, NewCoinRankInfo};
    use crate::infra::external::cgecko::DefaultCoinGecko;
    use crate::infra::external::cgecko::coin_rank::CoinRank;
    use crate::trace_fields;
    use bigdecimal::BigDecimal;
    use tracing::{error, info};

    #[tokio::test]
    async fn test_get_coin_rank() {
        listen_tracing::setup_tracing();
        let dcg = DefaultCoinGecko::default();
        let conin_list = dcg.get_coin_latest().await;

        let conin_rank_infos: Vec<NewCoinRankInfo> = conin_list.convert_vec();

        for coin_rank_info in &conin_rank_infos {
            info!(id= %coin_rank_info.id,
              name = %coin_rank_info.name,
              symbol = %coin_rank_info.symbol,
              current_price = %format_opt_decimal(&coin_rank_info.current_price),
              market_cap= %format_opt_decimal(&coin_rank_info.market_cap),
              market_cap_rank = &coin_rank_info.market_cap_rank,
               "coin_rank_info"
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
