use crate::collector::archive::fetch::kline_fetch_process;
use crate::collector::archive::flush::flush_all;
use crate::collector::archive::KlineMessage;
use crate::global::get_flush_buffer;
use crate::model::TimeFrame;
use std::collections::{HashMap, VecDeque};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::{mpsc, Mutex};
use tokio::task;
use tracing::info;

/// 每个任务
#[derive(Clone)]
struct ArchiveTaskEntry {
    symbol: String,
    exchange: String,
    time_frame: Arc<TimeFrame>,
    priority: u8, // 支持优先级，值越小优先级越高
}

/// 主调度器（生成任务 + 轮转调度）
pub async fn start_fair_task_scheduler() -> Result<(), anyhow::Error> {
    let (tx, rx) = mpsc::channel::<KlineMessage>(1000);
    let symbols = vec!["btcusdt"]; //, "ethusdt", "solusdt"
    let timeframes = vec![TimeFrame::M1]; //, TimeFrame::M5, TimeFrame::H1

    // 启动异步 worker pool
    tokio::spawn(start_worker_pool(rx, 20));

    // 启动调度器定时生成任务（公平轮询 + 优先级支持）
    let mut round_robin_queue = build_task_queues(&symbols, &timeframes);

    // 任务投递 这种方式每个任务执行一次
    tokio::spawn(async move {
        loop {
            let mut dispatched = false;

            for _ in 0..round_robin_queue.len() {
                if let Some(task_entry) = next_fair_task(&mut round_robin_queue) {
                    dispatched = true;
                    let tx = tx.clone();

                    task::spawn(async move {
                        let messages = kline_fetch_process(
                            task_entry.symbol.clone(),
                            task_entry.exchange.clone(),
                            task_entry.time_frame.clone(),
                        )
                        .await;

                        for msg in messages {
                            let _ = tx.send(msg).await;
                        }
                    });

                    tokio::time::sleep(Duration::from_millis(200)).await;
                }
            }

            // 所有任务已调度完成（空）
            if !dispatched {
                info!("All archive tasks have been dispatched.");
                break;
            }

            // 控制每轮调度间隔
            tokio::time::sleep(Duration::from_secs(2)).await;
        }
    });

    Ok(())
}

/// 构建轮转调度队列（每个任务配置优先级）
fn build_task_queues(
    symbols: &[&str],
    timeframes: &[TimeFrame],
) -> HashMap<u8, VecDeque<ArchiveTaskEntry>> {
    let mut map: HashMap<u8, VecDeque<ArchiveTaskEntry>> = HashMap::new();

    for symbol in symbols {
        for tf in timeframes {
            let priority = match tf {
                TimeFrame::M1 => 1,
                TimeFrame::M5 => 2,
                TimeFrame::H1 => 3,
                _ => 10,
            };
            map.entry(priority)
                .or_default()
                .push_back(ArchiveTaskEntry {
                    symbol: symbol.to_string(),
                    exchange: "binance".to_string(),
                    time_frame: Arc::new(tf.clone()),
                    priority,
                });
        }
    }

    map
}

/// 公平调度，按优先级轮转队列分发任务 会重复执行
// fn next_fair_task(
//     queues: &mut HashMap<u8, VecDeque<ArchiveTaskEntry>>,
// ) -> Option<ArchiveTaskEntry> {
//     let mut keys: Vec<u8> = queues.keys().cloned().collect();
//     keys.sort();
//
//     for k in keys {
//         if let Some(queue) = queues.get_mut(&k) {
//             if let Some(task) = queue.pop_front() {
//                 queue.push_back(task.clone()); // 重新放回队尾形成轮转
//                 return Some(task);
//             }
//         }
//     }
//     None
// }

/// 从多级队列中公平地弹出一个任务 让每个任务只执行一次
fn next_fair_task(
    queue_map: &mut HashMap<u8, VecDeque<ArchiveTaskEntry>>,
) -> Option<ArchiveTaskEntry> {
    let mut keys: Vec<u8> = queue_map.keys().cloned().collect();
    keys.sort(); // 从低数字的优先级开始

    for key in keys {
        if let Some(queue) = queue_map.get_mut(&key) {
            if let Some(task) = queue.pop_front() {
                return Some(task);
            }
        }
    }

    None
}

/// 启动异步 Worker 池，消费 `KlineMessage` 并按方向写入目标（MySQL/ClickHouse）
pub async fn start_worker_pool(receiver: mpsc::Receiver<KlineMessage>, num_workers: usize) {
    let receiver = Arc::new(Mutex::new(receiver));

    for i in 0..num_workers {
        let receiver = Arc::clone(&receiver);
        let buffer = get_flush_buffer(); // Forward 和 Backward 都共用一个 buffer，内部按方向区分

        tokio::spawn(async move {
            loop {
                // 尽量缩小锁粒度
                let maybe_msg = {
                    let mut locked = receiver.lock().await;
                    locked.recv().await
                };

                let Some(msg) = maybe_msg else {
                    break;
                };

                buffer.add(msg).await;

                // Flush 逻辑交给统一调度器处理
                if buffer.should_flush_forward().await || buffer.should_flush_backward().await {
                    if let Err(e) = flush_all(&*buffer).await {
                        tracing::error!("Worker {i} flush failed: {:?}", e);
                    }
                }
            }

            // 最后一轮 drain
            if let Err(e) = flush_all(&*buffer).await {
                tracing::error!("Worker {i} final flush failed: {:?}", e);
            }
        });
    }
}
