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
pub struct CoinRankInfoRepository<'a> {
    pub conn: &'a mut MysqlConnection,
}

impl<'a> CoinRankInfoRepository<'a> {
    pub fn new(conn: &'a mut MysqlConnection) -> Self {
        Self { conn }
    }
}

impl_full_repository!(
    CoinRankInfoRepository,  // Repository struct
    coin_rank_info,          // Table name from schema.rs
    CoinRankInfo,            // Model
    NewOrUpdateCoinRankInfo, // Insert model
    NewOrUpdateCoinRankInfo  // Update model
);

impl_repository_with_filter!(
    CoinRankInfoRepository,
    coin_rank_info,
    CoinRankInfo,
    CoinRankInfoFilter,
    @filter_var = filter,
    {
        use crate::schema::coin_rank_info::dsl::*;
        let mut q = coin_rank_info.into_boxed();

        if let Some(ref keyword) = filter.symbol_like {
            q = q.filter(symbol.like(format!("%{}%", keyword)));
        }

        if let Some(ref exact) = filter.symbol {
            q = q.filter(symbol.eq(exact));
        }

        if let Some(min) = filter.min_rank {
            q = q.filter(market_cap_rank.ge(min));
        }

        if let Some(max) = filter.max_rank {
            q = q.filter(market_cap_rank.le(max));
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
