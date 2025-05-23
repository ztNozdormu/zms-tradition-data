use crate::domain::model::AppResult;

pub mod coin_rank_info_repository;

// 定义 Repository trait
pub trait Repository<T> {
    fn get_all(&mut self) -> AppResult<Vec<T>>;
    fn get_by_id(&mut self, id: &str) -> AppResult<Option<T>>;
    fn insert(&mut self, entity: &T) -> AppResult<usize>;
    fn delete(&mut self, id: &str) -> AppResult<usize>;
}