use crate::domain::model::market_kline::{MarketKline, MarketKlineFilter, NewOrUpdateMarketKline};
use crate::domain::model::{AppError, AppResult, SortOrder};
use crate::domain::repository::Repository;
use crate::schema::market_kline::dsl::market_kline;
use crate::{impl_full_repository, impl_repository_with_filter};
use diesel::{MysqlConnection, OptionalExtension, QueryDsl, RunQueryDsl, SelectableHelper};
// market_kline_repository
pub struct MarketKlineRepository<'a> {
    pub conn: &'a mut MysqlConnection,
}

impl<'a> MarketKlineRepository<'a> {
    pub fn new(conn: &'a mut MysqlConnection) -> Self {
        Self { conn }
    }
}

impl_full_repository!(
    MarketKlineRepository,  // Repository struct
    market_kline,           // Table name from schema.rs
    MarketKline,            // Model
    NewOrUpdateMarketKline, // Insert model
    NewOrUpdateMarketKline  // Update model
);

impl_repository_with_filter!(
    MarketKlineRepository,
    market_kline,
    MarketKline,
    MarketKlineFilter,
    @filter_var = filter,
    {
        use crate::schema::market_kline::dsl::*;
        let mut q = market_kline.into_boxed();


        if let Some(ref exchange_arg) = filter.exchange {
            q = q.filter(exchange.eq(exchange_arg));
        }

        if let Some(ref symbol_arg) = filter.symbol {
            q = q.filter(symbol.eq(symbol_arg));
        }

        if let Some(ref time_frame_arg) = filter.time_frame {
            q = q.filter(time_frame.eq(time_frame_arg));
        }

         if let Some(order) = &filter.sort_by_close_time {
            q = {
                match order {
                    SortOrder::Asc => q.order(close_time.asc()),
                    SortOrder::Desc => q.order(close_time.desc()),
                }
            };
        }
        q
    }
);
