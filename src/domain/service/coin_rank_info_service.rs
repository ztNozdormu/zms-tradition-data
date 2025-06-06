use crate::common::VecConvert;
use crate::domain::model::coin_rank_info::{
    CoinRankInfo, CoinRankInfoFilter, NewOrUpdateCoinRankInfo,
};
use crate::domain::model::{AppResult, PageResult};
use crate::domain::repository::coin_rank_info_repository::CoinRankInfoRepository;
use crate::domain::repository::Repository;
use crate::domain::repository::UpdatableRepository;
use crate::domain::repository::{FilterableRepository, InsertableRepository};
use crate::impl_full_service;
use crate::infra::external::cgecko::DefaultCoinGecko;
use crate::schema::coin_rank_info;
use diesel::{Connection, MysqlConnection, RunQueryDsl};
use tracing::instrument;

impl_full_service!(
    CoinRankInfoService,
    CoinRankInfoRepository,
    CoinRankInfo,
    NewOrUpdateCoinRankInfo,
    NewOrUpdateCoinRankInfo
);

impl<'a> CoinRankInfoService<'a> {
    #[instrument(name = "save_coin_rank_info")]
    pub async fn save_coin_rank_info(&mut self) -> anyhow::Result<()> {
        let list = fetch_coin_rank_data().await;
        insert_or_update_coin_ranks(&mut self.repo.conn, list)?;
        Ok(())
    }

    pub fn query_page_with_total(
        &mut self,
        filter: CoinRankInfoFilter,
        page: i64,
        per_page: i64,
    ) -> AppResult<PageResult<CoinRankInfo>> {
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

/// 从 CoinGecko 获取并转换为结构化数据
async fn fetch_coin_rank_data() -> Vec<NewOrUpdateCoinRankInfo> {
    let dcg = DefaultCoinGecko::default();
    let raw_list = dcg.get_coin_rank().await;
    raw_list.convert_vec()
}

fn insert_or_update_coin_ranks(
    conn: &mut MysqlConnection,
    new_ranks: Vec<NewOrUpdateCoinRankInfo>,
) -> anyhow::Result<()> {
    conn.transaction(|conn| {
        for rank_info in &new_ranks {
            diesel::insert_into(coin_rank_info::table)
                .values(rank_info)
                .on_conflict(diesel::dsl::DuplicatedKeys)
                .do_update()
                .set(rank_info)
                .execute(conn)?;
        }
        Ok(())
    })
}
