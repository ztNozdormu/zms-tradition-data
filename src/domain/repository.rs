use crate::domain::model::AppResult;

pub mod coin_rank_info_repository;

// 定义 Repository trait
pub trait Repository<T> {
    fn get_all(&mut self) -> AppResult<Vec<T>>;
    fn get_by_id(&mut self, id: &str) -> AppResult<Option<T>>;
    fn delete(&mut self, id: &str) -> AppResult<usize>;
}

pub trait InsertableRepository<E> {
    fn insert(&mut self, entity: &E) -> AppResult<usize>;
}

pub trait UpdatableRepository<E> {
    fn update(&mut self, entity: &E) -> AppResult<usize>;
}

