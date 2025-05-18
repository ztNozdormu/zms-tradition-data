use std::env;
use std::time::Duration;
use diesel::mysql::MysqlConnection;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection, PoolError};
use dotenv::dotenv;

pub type MySqlPool = Pool<ConnectionManager<MysqlConnection>>;
pub type MySqlPooledConnection = PooledConnection<ConnectionManager<MysqlConnection>>;

/// Initialize MySQL connection pool using r2d2 + diesel
pub fn make_mysql_pool() -> Result<MySqlPool, PoolError> {
    // Load environment variables from `.env` (optional but helpful)
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let manager = ConnectionManager::<MysqlConnection>::new(database_url);

    // Customize pool settings if needed
    Pool::builder()
        .max_size(20)
        .min_idle(Some(5))
        .connection_timeout(Duration::from_secs(5))
        .idle_timeout(Some(Duration::from_secs(300)))
        .build(manager)

}
