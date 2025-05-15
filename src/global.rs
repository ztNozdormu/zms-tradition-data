use crate::infra::db::ckdb::ClickhouseDb;
use crate::infra::cache::flush_controller::FlushController;
use crate::infra::cache::kv_store::RedisKVStore;
use crate::common::utils::{make_binace_client, make_db, make_kv_store};
use barter::barter_xchange::exchange::binance::futures::market::FuturesMarket;
use once_cell::sync::OnceCell;
use std::sync::Arc;

// Use Arc to avoid cloning actual instances and allow shared ownership
pub static CK_DB: OnceCell<Arc<ClickhouseDb>> = OnceCell::new();
pub static KV_STORE: OnceCell<Arc<RedisKVStore>> = OnceCell::new();
pub static FUTURES_MARKET: OnceCell<Arc<FuturesMarket>> = OnceCell::new();
pub static FLUSH_CONTROLLER: OnceCell<Arc<FlushController>> = OnceCell::new();

pub async fn init_global_services() {
    let ck_db = make_db().await.expect("Failed to initialize ClickhouseDb");
    let redis_store = make_kv_store()
        .await
        .expect("Failed to initialize Redis Store");
    let futures_market = make_binace_client()
        .await
        .expect("Failed to initialize BinAceClient");
    // Initialize FlushController with parameters like batch size and max duration
    let flush_controller = Arc::new(FlushController::new(
        1000,
        std::time::Duration::from_secs(7200),
    ));

    let _ = CK_DB.set(ck_db);
    let _ = KV_STORE
        .set(redis_store)
        .expect("KV Store already initialized");
    let _ = FUTURES_MARKET.set(futures_market);

    let _ = FLUSH_CONTROLLER.set(flush_controller);
}

/// Get shared ClickHouse instance (panics if not initialized)
pub fn get_ck_db() -> Arc<ClickhouseDb> {
    CK_DB.get().expect("ClickhouseDb not initialized").clone()
}

/// Get shared KV store instance (panics if not initialized)
pub fn get_kv() -> Arc<RedisKVStore> {
    KV_STORE.get().expect("KvStore not initialized").clone()
}

/// Get shared KV store instance (panics if not initialized)
pub fn get_futures_market() -> Arc<FuturesMarket> {
    FUTURES_MARKET
        .get()
        .expect("KvStore not initialized")
        .clone()
}
/// Get shared FlushController instance (panics if not initialized)
pub fn get_flush_controller() -> Arc<FlushController> {
    FLUSH_CONTROLLER
        .get()
        .expect("FlushController not initialized")
        .clone()
}
