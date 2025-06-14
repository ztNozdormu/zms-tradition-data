use crate::infra::cache::kv_store::RedisKVStore;
use crate::infra::db::{ckdb::ClickhouseDb, types::ClickHouseDatabase};
use anyhow::Result;
use bb8_redis::{bb8, RedisConnectionManager};
use bigdecimal::BigDecimal;
use std::{fs::File, io::BufWriter, sync::Arc};

pub fn is_local() -> bool {
    std::env::var("LOCAL").is_ok()
}
pub async fn make_kv_store() -> Result<Arc<RedisKVStore>> {
    match is_local() {
        true => {
            let kv_store = RedisKVStore::new("redis://localhost:6379").await?;
            Ok(Arc::new(kv_store))
        }
        false => {
            let kv_store = RedisKVStore::new(must_get_env("REDIS_URL").as_str()).await?;
            Ok(Arc::new(kv_store))
        }
    }
}

pub async fn make_db() -> Result<Arc<ClickhouseDb>> {
    let mut db = match is_local() {
        true => ClickhouseDb::new("http://localhost:8123", "default", "default", "default"),
        false => ClickhouseDb::new(
            must_get_env("CLICKHOUSE_URL").as_str(),
            must_get_env("CLICKHOUSE_PASSWORD").as_str(),
            must_get_env("CLICKHOUSE_USER").as_str(),
            must_get_env("CLICKHOUSE_DATABASE").as_str(),
        ),
    };
    db.initialize().await?;
    Ok(Arc::new(db))
}

pub fn write_json(data: &str, file_name: &str) -> Result<()> {
    let file = File::create(file_name)?;
    let writer = BufWriter::new(file);
    serde_json::to_writer(writer, data)?;
    Ok(())
}

pub fn get_env_bool(key: &str, default: bool) -> bool {
    std::env::var(key)
        .map(|val| matches!(val.to_lowercase().as_str(), "1" | "true" | "yes"))
        .unwrap_or(default)
}

pub fn round_to_decimals(x: f64, decimals: u32) -> f64 {
    let y = 10i32.pow(decimals) as f64;
    (x * y).round() / y
}

pub fn must_get_env(key: &str) -> String {
    match std::env::var(key) {
        Ok(val) => val,
        Err(_) => panic!("{} must be set", key),
    }
}

pub async fn create_redis_pool(redis_url: &str) -> Result<bb8::Pool<RedisConnectionManager>> {
    let manager = RedisConnectionManager::new(redis_url)?;
    let pool = bb8::Pool::builder()
        .max_size(200)
        .min_idle(Some(20))
        .max_lifetime(Some(std::time::Duration::from_secs(60 * 15))) // 15 minutes
        .idle_timeout(Some(std::time::Duration::from_secs(60 * 5))) // 5 minutes
        .build(manager)
        .await?;
    Ok(pool)
}

pub fn format_opt_decimal(val: &Option<BigDecimal>) -> String {
    val.as_ref()
        .map(ToString::to_string)
        .unwrap_or_else(|| "N/A".to_string())
}

#[cfg(test)]
mod tests {

    #[tokio::test]
    async fn test_get_jup_price() {
        todo!()
    }
}
