use diesel::result::Error as DieselError;
use thiserror::Error;

pub(crate) mod coin_category;
pub(crate) mod coin_data_info;
pub(crate) mod coin_rank_info;



pub type AppResult<T> = Result<T, AppError>;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] DieselError),

    #[error("Not found")]
    NotFound,

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("Internal error: {0}")]
    Internal(String),
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::VecConvert;
    use crate::common::utils::format_opt_decimal;
    use crate::domain::model::coin_category::{CoinCategory, NewCoinCategory};
    use crate::domain::model::coin_data_info::NewCoinDataInfo;
    use crate::domain::model::coin_rank_info::{CoinRankInfo, NewCoinRankInfo};
    use crate::infra::external::cgecko::DefaultCoinGecko;
    use crate::infra::external::cgecko::coin_rank::CoinRank;
    use bigdecimal::BigDecimal;
    use listen_tracing::trace_kv;
    use listen_tracing::tracing_utils::{fmt_bigdecimal, fmt_json_value, fmt_naive_date};
    use tracing::{error, info};

    #[tokio::test]
    async fn test_get_coin_rank() {
        listen_tracing::setup_tracing();
        let dcg = DefaultCoinGecko::default();
        let conin_list = dcg.get_coin_rank().await;

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

        let coin_data_info: NewCoinDataInfo = coin_data.unwrap().into();

        trace_kv!(info,
             "id" => coin_data_info.id,
             "name" => coin_data_info.name,
             "symbol" => coin_data_info.symbol,
             "categories" => fmt_json_value(&coin_data_info.categories),
             "market_cap_rank" => fmt_bigdecimal(&coin_data_info.sentiment_votes_up_percentage),
             "genesis_date" => fmt_naive_date(&coin_data_info.genesis_date),
        );
    }

    #[tokio::test]
    async fn test_get_categories() {
        listen_tracing::setup_tracing();
        let dcg = DefaultCoinGecko::default();
        let categories = dcg.get_categories().await;

        let new_coin_category_list: Vec<NewCoinCategory> = categories.convert_vec();

        for new_coin_category in &new_coin_category_list {
            trace_kv!(info,
                 "id" => new_coin_category.id,
                 "name" => new_coin_category.name,
                 "market_cap" => format_opt_decimal(&new_coin_category.market_cap),
            );
        }
    }
}
