use crate::scheduler::tasks::history_data::KlineMessage;
use async_trait::async_trait;
use barter::barter_xchange::exchange::binance::model::KlineSummary;
use std::sync::Arc;
use tokio::sync::Mutex;
// 根据你项目的实际路径调整

pub const FLUSH_THRESHOLD: usize = 50_000;

// #[derive(Default, Clone, Debug)]
// pub struct KlineBuffer {
//     pub data: Arc<Mutex<Vec<KlineMessage>>>,
// }
//
// impl KlineBuffer {
//     pub fn new() -> Self {
//         Self {
//             data: Arc::new(Mutex::new(Vec::new())),
//         }
//     }
//
//     pub async fn add(&self, message: KlineMessage) {
//         let mut locked = self.data.lock().await;
//         locked.push(message);
//     }
//
//     pub async fn should_flush(&self) -> bool {
//         let locked = self.data.lock().await;
//         locked.iter().map(|m| m.datas.len()).sum::<usize>() >= FLUSH_THRESHOLD
//     }
//
//     pub async fn drain(&self) -> Vec<KlineMessage> {
//         let mut locked = self.data.lock().await;
//         std::mem::take(&mut *locked)
//     }
// }

/// 存储临时的 KlineMessage 列表，用于批量写入 ClickHouse 前缓冲
#[derive(Default, Clone, Debug)]
pub struct KlineBuffer {
    pub data: Arc<Mutex<Vec<KlineMessage>>>,
}

impl KlineBuffer {
    pub fn new() -> Self {
        Self {
            data: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// 添加单个消息
    pub async fn add(&self, message: KlineMessage) {
        let mut locked = self.data.lock().await;
        locked.push(message);

        // 可选：添加观测日志
        if locked.len() % 10_000 == 0 {
            tracing::debug!("KlineBuffer 当前缓冲大小: {}", locked.len());
        }
    }

    /// 判断是否需要触发 flush（根据累积的数据量）
    pub async fn should_flush(&self) -> bool {
        let locked = self.data.lock().await;
        locked.iter().map(|m| m.datas.len()).sum::<usize>() >= FLUSH_THRESHOLD
    }

    /// 提取所有缓冲数据并清空
    pub async fn drain(&self) -> Vec<KlineMessage> {
        let mut locked = self.data.lock().await;
        std::mem::take(&mut *locked)
    }
}

/// 提供统一 flush 缓冲行为接口，便于解耦 worker、测试、调度器等模块
#[async_trait]
pub trait FlushableBuffer: Send + Sync {
    async fn add(&self, msg: KlineMessage);
    async fn should_flush(&self) -> bool;
    async fn drain(&self) -> Vec<KlineMessage>;
}

#[async_trait]
impl FlushableBuffer for KlineBuffer {
    async fn add(&self, msg: KlineMessage) {
        self.add(msg).await;
    }

    async fn should_flush(&self) -> bool {
        self.should_flush().await
    }

    async fn drain(&self) -> Vec<KlineMessage> {
        self.drain().await
    }
}
