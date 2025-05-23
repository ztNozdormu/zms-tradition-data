use crate::common::VecConvert;
use crate::domain::model::coin_rank_info::{CoinRankInfo, NewCoinRankInfo};
use crate::infra::external::cgecko::DefaultCoinGecko;
use crate::schema::coin_rank_info;
use diesel::{Connection, MysqlConnection, RunQueryDsl};
use tracing::instrument;
use crate::domain::model::AppResult;
use crate::domain::repository::coin_rank_info_repository::CoinRankInfoRepository;
use crate::domain::repository::Repository;
pub struct CoinRankInfoService<'a> {
    pub(crate) repo: CoinRankInfoRepository<'a>,
}

impl<'a> std::fmt::Debug for CoinRankInfoService<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CoinRankInfoService")
            .field("field_name", &"<redacted>") // for sensitive fields
            .finish()
    }
}

impl<'a> CoinRankInfoService<'a> {
    pub fn new(conn: &'a mut MysqlConnection) -> Self {
        Self {
            repo: CoinRankInfoRepository::new(conn),
        }
    }
    // default order by rank
    pub fn fetch_all(&mut self) -> AppResult<Vec<CoinRankInfo>> {
        self.repo.get_all()
    }

    /// 主入口：获取并保存 Coin 排名数据
    #[instrument(name = "save_coin_rank_info")]
    pub async fn save_coin_rank_info(&mut self) -> Result<(), anyhow::Error> {
        let coin_rank_infos = fetch_coin_rank_data().await;
        insert_or_update_coin_ranks(&mut self.repo.conn, coin_rank_infos)?;

        Ok(())
    }

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
}

