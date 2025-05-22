pub mod fetch_cgecko;
pub mod clean_data;
pub mod notify_info;

use std::time::Duration;

/// 定义一个任务的调度信息
pub struct ScheduledTask {
    pub name: &'static str,
    pub interval: Duration,
    pub task_fn: fn() -> futures::future::BoxFuture<'static, Result<(), anyhow::Error>>,
}

/// 将异步函数转换为任务类型
macro_rules! task {
    ($name:expr, $interval:expr, $func:expr) => {
        ScheduledTask {
            name: $name,
            interval: $interval,
            task_fn: || Box::pin($func()),
        }
    };
}

/// 所有需要定时执行的任务列表
pub fn get_all_tasks() -> Vec<ScheduledTask> {
    vec![
        task!("save_coin_rank_info", Duration::from_secs(60), fetch_cgecko::save_coin_rank_info_task),
        // task!("clean_data", Duration::from_secs(300), clean_data),
        // task!("push_data", Duration::from_secs(120), push_data),
    ]
}
