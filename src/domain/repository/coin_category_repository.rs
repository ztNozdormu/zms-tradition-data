use crate::domain::model::coin_category::{
    CoinCategoriesFilter, CoinCategory, NewOrUpdateCoinCategory,
};
use crate::domain::model::{AppError, AppResult, SortOrder};
use crate::domain::repository::Repository;
use crate::schema::coin_categories::dsl::coin_categories;
use crate::{impl_full_repository, impl_repository_with_filter};
use diesel::{
    ExpressionMethods, MysqlConnection, OptionalExtension, QueryDsl, RunQueryDsl, SelectableHelper,
};

// coin_category_repository
pub struct CoinCategoryRepository<'a> {
    pub conn: &'a mut MysqlConnection,
}

impl<'a> CoinCategoryRepository<'a> {
    pub fn new(conn: &'a mut MysqlConnection) -> Self {
        Self { conn }
    }
}

impl_full_repository!(
    CoinCategoryRepository,  // Repository struct
    coin_categories,         // Table name from schema.rs
    CoinCategory,            // Model
    NewOrUpdateCoinCategory, // Insert model
    NewOrUpdateCoinCategory  // Update model
);

impl_repository_with_filter!(
    CoinCategoryRepository,
    coin_categories,
    CoinCategory,
    CoinCategoriesFilter,
    @filter_var = filter,
    {
        use crate::schema::coin_categories::dsl::*;
        let mut q = coin_categories.into_boxed();

        if let Some(ref keyword) = filter.name_like {
            q = q.filter(name.like(format!("%{}%", keyword)));
        }

        if let Some(ref exact) = filter.name {
            q = q.filter(name.eq(exact));
        }

        if let Some(ref mcp) = filter.market_cap {
            q = q.filter(market_cap.ge(mcp));
        }

        if let Some(ref vol_24h) = filter.volume_24h {
            q = q.filter(volume_24h.ge(vol_24h));
        }

         if let Some(order) = &filter.sort_by_rank {
            q = {
                match order {
                    SortOrder::Asc => q.order(market_cap.asc()),
                    SortOrder::Desc => q.order(market_cap.desc()),
                }
            };
        }
        q
    }
);
