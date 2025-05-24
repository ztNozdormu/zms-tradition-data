use crate::common::VecConvert;
use crate::domain::model::coin_category::{
    CoinCategoriesFilter, CoinCategory, NewOrUpdateCoinCategory,
};
use crate::domain::model::{AppResult, PageResult};
use crate::domain::repository::FilterableRepository;
use crate::domain::repository::InsertableRepository;
use crate::domain::repository::Repository;
use crate::domain::repository::UpdatableRepository;
use crate::domain::repository::coin_category_repository::CoinCategoryRepository;
use crate::impl_full_service;
use crate::infra::external::cgecko::DefaultCoinGecko;
use crate::schema::coin_categories;
use diesel::{Connection, MysqlConnection, RunQueryDsl};
use tracing::instrument;

impl_full_service!(
    CoinCategoryService,
    CoinCategoryRepository,
    CoinCategory,
    NewOrUpdateCoinCategory,
    NewOrUpdateCoinCategory
);

impl<'a> CoinCategoryService<'a> {
    /// 主入口：获取并保存 Coin 板块分类
    #[instrument(name = "save_categorys")]
    pub async fn save_categorys(&mut self) -> Result<(), anyhow::Error> {
        let coin_categories = fetch_coin_categories().await;

        insert_or_update_coin_categories(&mut self.repo.conn, coin_categories)?;

        Ok(())
    }
    pub fn query_page_with_total(
        &mut self,
        filter: CoinCategoriesFilter,
        page: i64,
        per_page: i64,
    ) -> AppResult<PageResult<CoinCategory>> {
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
async fn fetch_coin_categories() -> Vec<NewOrUpdateCoinCategory> {
    let dcg = DefaultCoinGecko::default();
    let categories = dcg.get_categories().await;
    categories.convert_vec()
}

fn insert_or_update_coin_categories(
    conn: &mut MysqlConnection,
    new_coin_categorys: Vec<NewOrUpdateCoinCategory>,
) -> anyhow::Result<()> {
    conn.transaction(|conn| {
        for new_coin_category in &new_coin_categorys {
            diesel::insert_into(coin_categories::table)
                .values(new_coin_category)
                .on_conflict(diesel::dsl::DuplicatedKeys)
                .do_update()
                .set(new_coin_category)
                .execute(conn)?;
        }
        Ok(())
    })
}
