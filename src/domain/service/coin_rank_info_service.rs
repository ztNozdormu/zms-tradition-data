use crate::common::VecConvert;
use crate::domain::model::coin_rank_info::NewCoinRankInfo;
use crate::global::get_mysql_pool;
use crate::infra::external::cgecko::DefaultCoinGecko;
use crate::schema::coin_rank_info;
use diesel::{Connection, MysqlConnection, RunQueryDsl};
use tracing::instrument;

/// 主入口：获取并保存 Coin 排名数据
#[instrument(name = "save_coin_rank_info")]
pub async fn save_coin_rank_info() -> anyhow::Result<()> {
    let coin_rank_infos = fetch_coin_rank_data().await;
    let mut conn = get_mysql_pool().get()?;

    insert_or_update_coin_ranks(&mut conn, coin_rank_infos)?;

    Ok(())
}

/// 从 CoinGecko 获取并转换为结构化数据
async fn fetch_coin_rank_data() -> Vec<NewCoinRankInfo> {
    let dcg = DefaultCoinGecko::default();
    let raw_list = dcg.get_coin_rank().await;
    raw_list.convert_vec()
}

fn insert_or_update_coin_ranks(
    conn: &mut MysqlConnection,
    new_ranks: Vec<NewCoinRankInfo>,
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

    // Ok(())
}

async fn get_coins_by_rank() {
    todo!()
}
