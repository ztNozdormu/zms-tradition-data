use crate::model::cex::kline::MarketKline;
use crate::model::dex::price::PriceUpdate;
use clickhouse::Row;
use clickhouse::inserter::Inserter;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;

pub trait TableRecord: Row + Sized + Send + Sync + Clone + 'static {
    const TABLE_NAME: &'static str;
    fn get_enum_inserter(inserter: &AnyInserter) -> Option<&RwLock<Inserter<Self>>>;
}

#[async_trait::async_trait]
pub trait ClickHouseDatabase {
    fn new(database_url: &str, password: &str, user: &str, database: &str) -> Self
    where
        Self: Sized;
    async fn initialize(&mut self) -> anyhow::Result<()>;

    async fn health_check(&self) -> anyhow::Result<()>;

    async fn insert<T>(&self, data: &T, commit: bool) -> anyhow::Result<()>
    where
        T: TableRecord + Row + Serialize;
    async fn insert_batch<T>(&self, data: &[T]) -> anyhow::Result<()>
    where
        T: TableRecord + Row + Serialize;

    async fn create_table_if_not_exists(
        &self,
        table_name: &str,
        create_query: &str,
    ) -> anyhow::Result<()>;
}

pub enum AnyInserter {
    PriceUpdate(Arc<RwLock<Inserter<PriceUpdate>>>),
    MarketKline(Arc<RwLock<Inserter<MarketKline>>>),
    // 其他表类型可继续添加
}

#[async_trait::async_trait]
pub trait Paginatable<T> {
    async fn get_paginated(
        &self,
        exchange: &str,
        symbol: &str,
        period: &str,
        params: &PageParams,
    ) -> anyhow::Result<PageResult<T>>;
}

#[derive(Debug, Clone)]
pub enum SortOrder {
    Asc,
    Desc,
}

/// 分页参数
#[derive(Debug, Clone)]
pub struct PageParams {
    pub limit: usize,
    pub offset: usize,
    pub sort_order: SortOrder,
    pub start_time: Option<u64>,
    pub end_time: Option<u64>,
}

/// 分页结果
#[derive(Debug, Clone)]
pub struct PageResult<T> {
    pub total: usize,
    pub items: Vec<T>,
}

#[derive(Debug, Deserialize, Row)]
pub struct RowCount {
    pub count: u64,
}
