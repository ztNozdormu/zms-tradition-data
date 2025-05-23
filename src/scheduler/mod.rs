pub mod tasks;

use std::time::Duration;
use tokio::{select, time};
use tracing::{error, info};

use crate::scheduler::tasks::{ScheduledTask, get_all_tasks};

pub struct Scheduler;

impl Scheduler {
    pub fn new() -> Self {
        Scheduler
    }

    pub async fn run(&self) {
        info!("Scheduler started...");

        let tasks = get_all_tasks();

        for task in tasks {
            tokio::spawn(run_periodic_task(task));
        }
    }
}

/// 单个任务的调度执行循环
async fn run_periodic_task(task: ScheduledTask) {
    let mut interval = time::interval(task.interval);
    loop {
        interval.tick().await;

        let task_name = task.name;
        let task_fn = task.task_fn;

        info!(task = task_name, "Running scheduled task...");
        let result = task_fn().await;

        if let Err(err) = result {
            error!(task = task_name, error = ?err, "Scheduled task failed");
        } else {
            info!(task = task_name, "Scheduled task completed");
        }
    }
}
