use crate::domain::model::market_symbol::{
    MarketSymbol, MarketSymbolFilter, NewOrUpdateMarketSymbol,
};
use crate::domain::model::{AppError, AppResult};
use crate::domain::repository::Repository;
use crate::{impl_full_repository, impl_repository_with_filter};
use diesel::{MysqlConnection, OptionalExtension, QueryDsl, RunQueryDsl, SelectableHelper};

// market_symbol_repository
pub struct MarketSymbolRepository<'a> {
    pub conn: &'a mut MysqlConnection,
}

impl<'a> MarketSymbolRepository<'a> {
    pub fn new(conn: &'a mut MysqlConnection) -> Self {
        Self { conn }
    }
}

impl_full_repository!(
    MarketSymbolRepository,  // Repository struct
    market_symbol,           // Table name from schema.rs
    MarketSymbol,            // Model
    NewOrUpdateMarketSymbol, // Insert model
    NewOrUpdateMarketSymbol  // Update model
);

impl_repository_with_filter!(
    MarketSymbolRepository,
    market_symbol,
    MarketSymbol,
    MarketSymbolFilter,
    @filter_var = filter,
    {
        use crate::schema::market_symbol::dsl::*;
        let mut q = market_symbol.into_boxed();


        if let Some(ref exchange_arg) = filter.exchange {
            q = q.filter(exchange.eq(exchange_arg));
        }

        if let Some(ref symbol_arg) = filter.symbol {
            q = q.filter(symbol.eq(symbol_arg));
        }
        q
    }
);
