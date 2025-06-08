use crate::domain::model::market_kline::{MarketKline, MarketKlineFilter, NewOrUpdateMarketKline};
use crate::domain::model::{AppResult, PageResult};
use crate::domain::repository::market_kline_repository::MarketKlineRepository;
use crate::domain::repository::Repository;
use crate::domain::repository::UpdatableRepository;
use crate::domain::repository::{FilterableRepository, InsertableRepository};
use crate::impl_full_service;
use crate::model::cex::kline::MinMaxCloseTime;
use crate::schema::market_kline;
use anyhow::Result;
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
    #[instrument(name = "save_market_klines")]
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

    #[instrument(name = "save_coin_rank_info")]
    pub async fn save_coin_rank_info(&mut self, datas: Vec<NewOrUpdateMarketKline>) -> Result<()> {
        insert_or_update_market_klines(&mut self.repo.conn, datas)?;
        Ok(())
    }

    /// 查询指定交易所、币对、周期的最早和最晚时间
    pub async fn get_mima_time(
        &mut self,
        exchange_val: &str,
        symbol_val: &str,
        time_frame_val: &str,
    ) -> Result<Option<MinMaxCloseTime>, diesel::result::Error> {
        use crate::schema::market_kline::dsl::*;
        use diesel::dsl::{max, min};
        use diesel::prelude::*;

        let result: Option<(Option<i64>, Option<i64>)> = market_kline
            .filter(exchange.eq(exchange_val))
            .filter(symbol.eq(symbol_val))
            .filter(time_frame.eq(time_frame_val))
            .select((min(close_time), max(close_time)))
            .first::<(Option<i64>, Option<i64>)>(self.repo.conn)
            .optional()?; // 返回 Ok(None) 如果没有行

        // 统一处理为 Some(0, 0) 即使没有数据
        let (min, max) = result.unwrap_or((Some(0), Some(0)));
        Ok(Some(MinMaxCloseTime {
            min_close_time: min.unwrap_or(0),
            max_close_time: max.unwrap_or(0),
        }))
    }
}

fn insert_or_update_market_klines(
    conn: &mut MysqlConnection,
    new_ranks: Vec<NewOrUpdateMarketKline>,
) -> Result<()> {
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
