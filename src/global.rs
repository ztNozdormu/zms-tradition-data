use crate::collector::archive::kline_buffer::KlineBuffer;
use crate::common::utils::{make_db, make_kv_store};
use crate::infra::cache::flush_controller::FlushController;
use crate::infra::cache::kv_store::RedisKVStore;
use crate::infra::db::ckdb::ClickhouseDb;
use crate::infra::db::mysql::{make_mysql_pool, MySqlPool};
use once_cell::sync::OnceCell;
use std::sync::Arc;

// Use Arc to avoid cloning actual instances and allow shared ownership
pub static CK_DB: OnceCell<Arc<ClickhouseDb>> = OnceCell::new();
pub static KV_STORE: OnceCell<Arc<RedisKVStore>> = OnceCell::new();
pub static MYSQL_POOL: OnceCell<Arc<MySqlPool>> = OnceCell::new();
pub static FLUSH_CONTROLLER: OnceCell<Arc<FlushController>> = OnceCell::new();
pub static FLUSH_BUFFER: OnceCell<Arc<KlineBuffer>> = OnceCell::new();

pub async fn init_global_services() {
    let ck_db = make_db().await.expect("Failed to initialize ClickhouseDb");
    let redis_store = make_kv_store()
        .await
        .expect("Failed to initialize Redis Store");
    let mysql_pool = Arc::new(make_mysql_pool().expect("MySQL init failed"));

    // Initialize FlushController with parameters like batch size and max duration
    let flush_controller = Arc::new(FlushController::new(
        1000,
        std::time::Duration::from_secs(7200),
    ));

    let flush_buffer = Arc::new(KlineBuffer::new());

    let _ = set_ck_db(ck_db);
    let _ = set_kv_store(redis_store).unwrap();
    let _ = set_mysql_pool(mysql_pool).unwrap();
    let _ = set_flush_controller(flush_controller);
    let _ = set_flush_buffer(flush_buffer);
}

pub fn set_ck_db(instance: Arc<ClickhouseDb>) -> Result<(), Arc<ClickhouseDb>> {
    CK_DB.set(instance)
}

pub fn set_kv_store(instance: Arc<RedisKVStore>) -> Result<(), Arc<RedisKVStore>> {
    KV_STORE.set(instance)
}

pub fn set_mysql_pool(instance: Arc<MySqlPool>) -> Result<(), Arc<MySqlPool>> {
    MYSQL_POOL.set(instance)
}

pub fn set_flush_controller(instance: Arc<FlushController>) -> Result<(), Arc<FlushController>> {
    FLUSH_CONTROLLER.set(instance)
}

pub fn set_flush_buffer(instance: Arc<KlineBuffer>) -> Result<(), Arc<KlineBuffer>> {
    FLUSH_BUFFER.set(instance)
}

/// Get shared ClickHouse instance (panics if not initialized)
pub fn get_ck_db() -> Arc<ClickhouseDb> {
    CK_DB.get().expect("ClickhouseDb not initialized").clone()
}

/// Get shared KV store instance (panics if not initialized)
pub fn get_kv() -> Arc<RedisKVStore> {
    KV_STORE.get().expect("KvStore not initialized").clone()
}

pub fn get_mysql_pool() -> Arc<MySqlPool> {
    MYSQL_POOL
        .get()
        .expect("MYSQL_POOL not initialized")
        .clone()
}

/// Get shared FlushController instance (panics if not initialized)
pub fn get_flush_controller() -> Arc<FlushController> {
    FLUSH_CONTROLLER
        .get()
        .expect("FlushController not initialized")
        .clone()
}

pub fn get_flush_buffer() -> Arc<KlineBuffer> {
    FLUSH_BUFFER
        .get()
        .expect("FLUSH_BUFFER not initialized")
        .clone()
}
