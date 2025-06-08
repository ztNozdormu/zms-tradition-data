pub mod fetch_cgecko;
pub mod history_data;
pub mod notify_info;

use std::time::Duration;

/// 定义一个任务的调度信息
pub struct ScheduledTask {
    pub name: &'static str,
    pub interval: Duration,
    pub task_fn: fn() -> futures::future::BoxFuture<'static, Result<(), anyhow::Error>>,
    pub once: bool,
}

/// 将异步函数转换为任务类型
macro_rules! task {
    ($name:expr, $interval:expr, $func:expr) => {
        ScheduledTask {
            name: $name,
            interval: $interval,
            task_fn: || Box::pin($func()),
            once: false,
        }
    };
}

macro_rules! once_task {
    ($name:expr, $func:expr) => {
        ScheduledTask {
            name: $name,
            interval: Duration::from_secs(0),
            task_fn: || Box::pin($func()),
            once: true,
        }
    };
}

/// 所有需要定时执行的任务列表
pub fn get_all_tasks() -> Vec<ScheduledTask> {
    vec![
        // once_task!("init_coin_rank", history_data::save_binance_symbol), 线程冲突 todo
        // every three days execute
        task!(
            "save_binance_symbol",
            Duration::from_secs(259200),
            history_data::save_binance_symbol
        ),
        task!(
            "save_coin_rank_info",
            Duration::from_secs(259200),
            fetch_cgecko::save_coin_rank_info_task
        ),
        task!(
            "save_coin_category_info",
            Duration::from_secs(259200),
            fetch_cgecko::save_categorys_task
        ),
        task!(
            "save_coin_data_info",
            Duration::from_secs(259200),
            fetch_cgecko::save_coin_data_info_task
        ),
        // 双向追溯
        task!(
            "sync_exchange_history_data",
            Duration::from_secs(3600),
            history_data::exchange_history_data
        ),
        // todo 定期将最新数据合并到clickhouse mysql只保留近三个月数据
        // todo 定期数据清洗
    ]
}
