use crate::domain::model::coin_data_info::{
    CoinDataInfo, CoinDataInfoFilter, NewOrUpdateCoinDataInfo,
};
use crate::domain::model::coin_rank_info::{
    CoinRankInfo, CoinRankInfoFilter, NewOrUpdateCoinRankInfo,
};
use crate::domain::model::{AppError, AppResult, SortOrder};
use crate::domain::repository::Repository;
use crate::schema::coin_rank_info::dsl::coin_rank_info;
use crate::{impl_full_repository, impl_repository_with_filter};
use diesel::{
    ExpressionMethods, MysqlConnection, OptionalExtension, QueryDsl, RunQueryDsl, SelectableHelper,
};

// coin_rank_info_repository
pub struct CoinDataInfoRepository<'a> {
    pub conn: &'a mut MysqlConnection,
}

impl<'a> CoinDataInfoRepository<'a> {
    pub fn new(conn: &'a mut MysqlConnection) -> Self {
        Self { conn }
    }
}

impl_full_repository!(
    CoinDataInfoRepository,  // Repository struct
    coin_data_info,          // Table name from schema.rs
    CoinDataInfo,            // Model
    NewOrUpdateCoinDataInfo, // Insert model
    NewOrUpdateCoinDataInfo  // Update model
);

impl_repository_with_filter!(
    CoinDataInfoRepository,
    coin_data_info,
    CoinDataInfo,
    CoinDataInfoFilter,
    @filter_var = filter,
    {
        use crate::schema::coin_data_info::dsl::*;
        let mut q = coin_data_info.into_boxed();

        if let Some(ref name_like_arg) = filter.name_like {
            q = q.filter(name.like(format!("%{}%", name_like_arg)));
        }

        if let Some(ref name_arg) = filter.name {
            q = q.filter(name.eq(name));
        }

        if let Some(ref symbol_arg) = filter.symbol {
            q = q.filter(symbol.eq(symbol_arg));
        }

        if let Some(rank) = filter.market_cap_rank {
            q = q.filter(market_cap_rank.ge(rank));
        }

         if let Some(order) = &filter.sort_by_rank {
            q = {
                match order {
                    SortOrder::Asc => q.order(market_cap_rank.asc()),
                    SortOrder::Desc => q.order(market_cap_rank.desc()),
                }
            };
        }
        q
    },
    composite_pk = [id]
);
