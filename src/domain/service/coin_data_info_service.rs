use crate::domain::model::coin_data_info::{
    CoinDataInfo, CoinDataInfoFilter, NewOrUpdateCoinDataInfo,
};
use crate::domain::model::{AppResult, PageResult};
use crate::domain::repository::coin_data_info_repository::CoinDataInfoRepository;
use crate::domain::repository::Repository;
use crate::domain::repository::{FilterableRepository, InsertableRepository, UpdatableRepository};
use crate::global::get_mysql_pool;
use crate::impl_full_service;
use crate::infra::external::cgecko::DefaultCoinGecko;
use crate::schema::coin_data_info;
use diesel::{Connection, MysqlConnection, RunQueryDsl};
use tracing::{info, instrument};

impl_full_service!(
    CoinDataInfoService,
    CoinDataInfoRepository,
    CoinDataInfo,
    NewOrUpdateCoinDataInfo,
    NewOrUpdateCoinDataInfo
);

impl<'a> CoinDataInfoService<'a> {
    /// 主入口：获取并保存 Coin_data_info 信息包含所属板块
    #[instrument(name = "save_coin_data_info")]
    pub async fn save_coin_data_info(&mut self, coin_id: &str) -> Result<(), anyhow::Error> {
        if let Ok(data_info) = fetch_coin_data_info(coin_id).await {
            insert_or_update_coin_data_info(&mut self.repo.conn, &data_info)?;
        } else {
            // 记录错误或忽略
            info!("Coin {} not found", coin_id);
        }
        Ok(())
    }

    pub fn query_page_with_total(
        &mut self,
        filter: CoinDataInfoFilter,
        page: i64,
        per_page: i64,
    ) -> AppResult<PageResult<CoinDataInfo>> {
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
pub async fn fetch_coin_data_info(coin_id: &str) -> Result<NewOrUpdateCoinDataInfo, anyhow::Error> {
    let dcg = DefaultCoinGecko::default();
    let coin_data = dcg.get_coin_data(coin_id).await;
    // info!("Coin {} data: {:?}", coin_id, coin_data);
    let coin_data_res =
        coin_data.ok_or_else(|| anyhow::anyhow!("Coin data not found: {coin_id}"))?;
    Ok(coin_data_res.into())
}
fn insert_or_update_coin_data_info(
    conn: &mut MysqlConnection,
    new_coin_data_info: &NewOrUpdateCoinDataInfo,
) -> anyhow::Result<()> {
    conn.transaction(|conn| {
        diesel::insert_into(coin_data_info::table)
            .values(new_coin_data_info)
            .on_conflict(diesel::dsl::DuplicatedKeys)
            .do_update()
            .set(new_coin_data_info)
            .execute(conn)?;
        Ok(())
    })
}
