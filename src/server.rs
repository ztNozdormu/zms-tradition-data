use std::env;
use std::fs::File;
use std::net::SocketAddr;
use std::sync::Arc;
use listen_tracing::{LogCache, LogEntry};
use tokio::spawn;
use tokio::sync::broadcast;
use warp::Filter;

mod routes;
mod response;

const APPLICATION_NAME: &str ="ZMS_MINI_BOT";
const FILE_PATH_SYSTEM_CONFIG: &str = "config/system_config.json";


#[derive(Clone)]
pub struct AppState{
    tx: broadcast::Sender<LogEntry>,
    cache: LogCache
}

pub async fn start() {
    // 创建广播通道用于实时日志
    let (tx, _) = broadcast::channel::<LogEntry>(1024);

    // 创建共享缓存用于历史日志查询
    let cache: LogCache = Arc::new(tokio::sync::RwLock::new(Vec::new()));

    // 初始化 tracing 日志系统
    listen_tracing::setup_tracing_with_broadcast(tx.clone(), cache.clone());

    let bind_address: SocketAddr = "127.0.0.1:10099".parse().unwrap();



    // init app
    let app_state = AppState{tx:tx.clone(), cache: cache.clone()};

    let routes = routes::routes(app_state).with(warp::log(APPLICATION_NAME));

    warp::serve(routes).run(bind_address).await;

    tracing::info!("You can access the server at {}", bind_address);
}

fn get_absolute_path(file_name: &str) -> String {
    let current_dir = env::current_dir().expect("无法获取当前目录");
    current_dir.join(file_name).to_str().unwrap().to_string()
}
