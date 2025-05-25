use crate::domain::model::market_kline::{MarketKline, MarketKlineFilter, NewOrUpdateMarketKline};
use crate::domain::model::{AppResult, PageResult};
use crate::domain::repository::Repository;
use crate::domain::repository::UpdatableRepository;
use crate::domain::repository::market_kline_repository::MarketKlineRepository;
use crate::domain::repository::{FilterableRepository, InsertableRepository};
use crate::impl_full_service;
use crate::schema::market_kline;
use diesel::{Connection, MysqlConnection, RunQueryDsl};
use tracing::instrument;

impl_full_service!(
    MarketKlineService,
    MarketKlineRepository,
    MarketKline,
    NewOrUpdateMarketKline,
    NewOrUpdateMarketKline
);

impl<'a> MarketKlineService<'a> {
    #[instrument(name = "save_coin_rank_info")]
    pub async fn exchange_history_data(&mut self) -> anyhow::Result<()> {
        todo!()
    }

    pub fn query_page_with_total(
        &mut self,
        filter: MarketKlineFilter,
        page: i64,
        per_page: i64,
    ) -> AppResult<PageResult<MarketKline>> {
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

/// 从 交易所(币安) 获取历史k线数据
async fn fetch_exchange_history_data() -> Vec<NewOrUpdateMarketKline> {
    todo!()
}

fn insert_or_update_market_klines(
    conn: &mut MysqlConnection,
    new_ranks: Vec<NewOrUpdateMarketKline>,
) -> anyhow::Result<()> {
    conn.transaction(|conn| {
        for market_kline in &new_ranks {
            diesel::insert_into(market_kline::table)
                .values(market_kline)
                .on_conflict(diesel::dsl::DuplicatedKeys)
                .do_update()
                .set(market_kline)
                .execute(conn)?;
        }
        Ok(())
    })
}
