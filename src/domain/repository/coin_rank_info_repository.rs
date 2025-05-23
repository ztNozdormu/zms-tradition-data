use crate::domain::model::coin_rank_info::{CoinRankInfo, NewCoinRankInfo};
use crate::domain::model::{AppError, AppResult};
use crate::domain::repository::Repository;
use crate::impl_full_repository;
use crate::schema::coin_rank_info::dsl::coin_rank_info;
use diesel::{MysqlConnection, OptionalExtension, QueryDsl, RunQueryDsl, SelectableHelper};

// coin_rank_info_repository
pub struct CoinRankInfoRepository<'a> {
    pub conn: &'a mut MysqlConnection,
}

impl<'a> CoinRankInfoRepository<'a> {
    pub fn new(conn: &'a mut MysqlConnection) -> Self {
        Self { conn }
    }
}

impl_full_repository!(
    CoinRankInfoRepository,       // Repository struct
    coin_rank_info,               // Table name from schema.rs
    CoinRankInfo,                 // Model
    NewCoinRankInfo,              // Insert model
    NewCoinRankInfo               // Update model
);

// pub struct CoinRankInfoRepository<'a> {
//     pub(crate) conn: &'a mut MysqlConnection,
// }
//
// impl<'a> CoinRankInfoRepository<'a> {
//     pub fn new(conn: &'a mut MysqlConnection) -> Self {
//         Self { conn }
//     }
// }
//
// impl<'a> Repository<CoinRankInfo> for CoinRankInfoRepository<'a> {
//     fn get_all(&mut self) -> AppResult<Vec<CoinRankInfo>> {
//         coin_rank_info::table()
//             .select(CoinRankInfo::as_select())
//             .load(self.conn)
//             .map_err(AppError::from)
//     }
//
//     fn get_by_id(&mut self, id: &str) -> AppResult<Option<CoinRankInfo>> {
//         coin_rank_info::table()
//             .find(id)
//             .select(CoinRankInfo::as_select())
//             .first(self.conn)
//             .optional()
//             .map_err(AppError::from)
//     }
//
//     fn insert(&mut self, entity: &CoinRankInfo) -> AppResult<usize> {
//         diesel::insert_into(coin_rank_info::table())
//             .values(entity)
//             .execute(self.conn)
//             .map_err(AppError::from)  // Convert QueryResult to AppResult
//     }
//
//     fn delete(&mut self, id: &str) -> AppResult<usize> {
//         diesel::delete(coin_rank_info::table().find(id)).execute(self.conn).map_err(AppError::from) // Convert QueryResult to AppResult
//     }
//
// }
