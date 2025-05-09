use once_cell::sync::OnceCell;
use std::sync::Arc;
use crate::db::ckdb::ClickhouseDb;
use crate::db::kv_store::RedisKVStore;
use crate::util::{make_db, make_kv_store};

// Use Arc to avoid cloning actual instances and allow shared ownership
pub static CK_DB: OnceCell<Arc<ClickhouseDb>> = OnceCell::new();
pub static KV_STORE: OnceCell<Arc<RedisKVStore>> = OnceCell::new();

pub async fn init_global_services() {
    let ck_db = make_db().await.expect("Failed to initialize ClickhouseDb");
    let redis_store = make_kv_store().await.expect("Failed to initialize Redis Store");

    let _ = CK_DB.set(ck_db);
    KV_STORE.set(redis_store).expect("KV Store already initialized");
}

/// Get shared ClickHouse instance (panics if not initialized)
pub fn get_db() -> Arc<ClickhouseDb> {
    CK_DB.get().expect("ClickhouseDb not initialized").clone()
}

/// Get shared KV store instance (panics if not initialized)
pub fn get_kv() -> Arc<RedisKVStore> {
    KV_STORE.get().expect("KvStore not initialized").clone()
}
