// get_categorys
// get_category_by_id

use crate::common::VecConvert;
use crate::domain::model::coin_category::NewCoinCategory;
use crate::global::get_mysql_pool;
use crate::infra::external::cgecko::DefaultCoinGecko;
use crate::schema::coin_categories;
use diesel::{Connection, MysqlConnection, RunQueryDsl};
use tracing::instrument;

/// 主入口：获取并保存 Coin 板块分类
#[instrument(name = "save_categorys")]
pub async fn save_categorys() -> anyhow::Result<()> {
    let coin_categories = fetch_coin_categories().await;
    let mut conn = get_mysql_pool().get()?;

    insert_or_update_coin_categories(&mut conn, coin_categories)?;

    Ok(())
}

/// 从 CoinGecko 获取并转换为结构化数据
async fn fetch_coin_categories() -> Vec<NewCoinCategory> {
    let dcg = DefaultCoinGecko::default();
    let categories = dcg.get_categories().await;
    categories.convert_vec()
}

fn insert_or_update_coin_categories(
    conn: &mut MysqlConnection,
    new_coin_categorys: Vec<NewCoinCategory>,
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
