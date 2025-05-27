use crate::collector::archive::kline_buffer::{FlushableBuffer, KlineBuffer};
use crate::global::{get_ck_db, get_flush_buffer};
use crate::infra::db::types::ClickHouseDatabase;
use crate::model::cex::kline::MarketKline;
use crate::scheduler::tasks::history_data::KlineMessage;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::sync::mpsc::{self, Sender};

pub struct TaskContext {
    pub buffer: KlineBuffer,
    pub sender: Sender<KlineMessage>,
}

// pub async fn start_worker_pool(
//     mut receiver: mpsc::Receiver<KlineMessage>,
//     num_workers: usize,
// ) {
//
//     for _ in 0..num_workers {
//         let buffer = get_flush_buffer();
//
//         tokio::spawn(async move {
//             while let Some(message) = receiver.recv().await {
//                 buffer.add(message).await;
//
//                 if buffer.should_flush().await {
//                     let messages = buffer.drain().await;
//                     let all_data: Vec<_> = messages
//                         .iter()
//                         .flat_map(|m| build_by_kline_message(m))
//                         .collect();
//
//                     if !all_data.is_empty() {
//                         let _ = get_ck_db().insert_batch(&all_data).await;
//                     }
//                 }
//             }
//
//             // 收完后最后一次 flush
//             let messages = buffer.drain().await;
//             let all_data: Vec<_> = messages
//                 .iter()
//                 .flat_map(|m| build_by_kline_message(m))
//                 .collect();
//
//             if !all_data.is_empty() {
//                 let _ = get_ck_db().insert_batch(&all_data).await;
//             }
//
//             // 🚀 此处你可以拿到每个 message 的上下文：
//             for message in messages {
//                 println!(
//                     "Flushed from symbol={} exchange={} tf={}",
//                     &message.symbol, &message.exchange, &message.time_frame
//                 );
//             }
//         });
//     }
// }

/// 启动异步 Worker 池，统一消费 `KlineMessage` 并批量写入 ClickHouse
pub async fn start_worker_pool(receiver: mpsc::Receiver<KlineMessage>, num_workers: usize) {
    let receiver = Arc::new(Mutex::new(receiver));

    for i in 0..num_workers {
        let receiver = Arc::clone(&receiver);
        let buffer = get_flush_buffer();

        tokio::spawn(async move {
            loop {
                // 仅锁住接收器以获取消息
                let maybe_msg = {
                    let mut locked = receiver.lock().await;
                    locked.recv().await
                };

                let Some(msg) = maybe_msg else {
                    break; // 所有 sender 已关闭，退出 loop
                };

                buffer.add(msg).await;

                if buffer.should_flush().await {
                    if let Err(e) = flush_buffer(&*buffer).await {
                        eprintln!("Worker {i} flush error: {:?}", e);
                    }
                }
            }

            // channel 已关闭，执行最后 flush
            if let Err(e) = flush_buffer(&*buffer).await {
                eprintln!("Worker {i} final flush error: {:?}", e);
            }
        });
    }
}

/// 执行一次 flush，将 buffer 中的数据转换并写入 ClickHouse
pub async fn flush_buffer<B>(buffer: &B) -> Result<(), anyhow::Error>
where
    B: FlushableBuffer + ?Sized,
{
    if !buffer.should_flush().await {
        return Ok(());
    }

    let messages = buffer.drain().await;
    let all_data: Vec<_> = messages
        .iter()
        .flat_map(|m| build_by_kline_message(m))
        .collect();

    if !all_data.is_empty() {
        get_ck_db().insert_batch(&all_data).await?;
    }

    Ok(())
}

/// 构建 MarketKline，带上下文信息
pub fn build_by_kline_message(message: &KlineMessage) -> Vec<MarketKline> {
    let KlineMessage {
        datas,
        exchange,
        symbol,
        time_frame,
    } = message;

    datas
        .iter()
        .map(|k| (k, exchange.as_str(), symbol.as_str(), time_frame.as_str()).into())
        .collect()
}
