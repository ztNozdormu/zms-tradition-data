use crate::domain::model::coin_data_info::NewCoinDataInfo;
use crate::global::get_mysql_pool;
use crate::infra::external::cgecko::DefaultCoinGecko;
use crate::schema::coin_data_info;
use diesel::{Connection, MysqlConnection, RunQueryDsl};
use tracing::instrument;

// get_coin_by_id
/// 主入口：获取并保存 Coin_data_info 信息包含所属板块
#[instrument(name = "save_coin_data_info")]
pub async fn save_coin_data_info(coin_id: &str) -> anyhow::Result<()> {
    let new_coin_data_info = fetch_coin_data_info(coin_id).await;
    let mut conn = get_mysql_pool().get()?;

    insert_or_update_coin_data_info(&mut conn, &new_coin_data_info)?;

    Ok(())
}

/// 从 CoinGecko 获取并转换为结构化数据
async fn fetch_coin_data_info(coin_id: &str) -> NewCoinDataInfo {
    let dcg = DefaultCoinGecko::default();
    let coin_data = dcg.get_coin_data(coin_id).await;
    coin_data.unwrap().into()
}

fn insert_or_update_coin_data_info(
    conn: &mut MysqlConnection,
    new_coin_data_info: &NewCoinDataInfo,
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
