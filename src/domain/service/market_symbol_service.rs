use crate::common::VecConvert;
use crate::domain::model::market_symbol::{
    MarketSymbol, MarketSymbolFilter, NewOrUpdateMarketSymbol,
};
use crate::domain::model::{AppResult, PageResult};
use crate::domain::repository::Repository;
use crate::domain::repository::UpdatableRepository;
use crate::domain::repository::market_symbol_repository::MarketSymbolRepository;
use crate::domain::repository::{FilterableRepository, InsertableRepository};
use crate::global::get_futures_general;
use crate::impl_full_service;
use crate::infra::external::binance::DefaultBinanceExchange;
use crate::infra::external::cgecko::DefaultCoinGecko;
use crate::schema::market_symbol;
use diesel::{Connection, IntoSql, MysqlConnection, RunQueryDsl};
use tracing::instrument;

impl_full_service!(
    MarketSymbolService,
    MarketSymbolRepository,
    MarketSymbol,
    NewOrUpdateMarketSymbol,
    NewOrUpdateMarketSymbol
);

impl<'a> MarketSymbolService<'a> {
    #[instrument(name = "save_exchange_symbol_info")]
    pub async fn save_exchange_symbol_info(&mut self) -> anyhow::Result<()> {
        let list = fetch_exchange_symbol_data().await;
        insert_or_update_market_symbols(&mut self.repo.conn, list)?;
        Ok(())
    }

    pub fn query_page_with_total(
        &mut self,
        filter: MarketSymbolFilter,
        page: i64,
        per_page: i64,
    ) -> AppResult<PageResult<MarketSymbol>> {
        let data = self.repo.filter_paginated(&filter, page, per_page)?;
        let total = self.repo.count_filtered(&filter)?;
        Ok(PageResult {
            data,
            total,
            page,
            per_page,
        })
    }
}

/// 从 交易所(币安) 获取币种数据
async fn fetch_exchange_symbol_data() -> Vec<NewOrUpdateMarketSymbol> {
    let dbe = DefaultBinanceExchange::default();
    if let Some(symbols) = dbe.get_symbols().await {
        symbols.convert_vec()
    } else {
        // 处理 None 的情况
        Vec::new()
    }
}

fn insert_or_update_market_symbols(
    conn: &mut MysqlConnection,
    new_symbols: Vec<NewOrUpdateMarketSymbol>,
) -> anyhow::Result<()> {
    conn.transaction(|conn| {
        for market_symbol in &new_symbols {
            diesel::insert_into(market_symbol::table)
                .values(market_symbol)
                .on_conflict(diesel::dsl::DuplicatedKeys)
                .do_update()
                .set(market_symbol)
                .execute(conn)?;
        }
        Ok(())
    })
}
