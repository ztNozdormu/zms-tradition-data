use crate::collector::archive::types::ArchiveDirection;
use crate::collector::archive::KlineMessage;
use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::Mutex;

pub const FORWARD_FLUSH_THRESHOLD: usize = 100;//1_000;
pub const BACKWARD_FLUSH_THRESHOLD: usize = 1000;// 50_000;

#[derive(Default, Clone, Debug)]
pub struct KlineBuffer {
    pub forward_data: Arc<Mutex<Vec<KlineMessage>>>,
    pub backward_data: Arc<Mutex<Vec<KlineMessage>>>,
}

impl KlineBuffer {
    pub fn new() -> Self {
        Self::default()
    }

    pub async fn add(&self, message: KlineMessage) {
        match message.archive_direction {
            ArchiveDirection::Forward => {
                self.forward_data.lock().await.push(message);
            }
            ArchiveDirection::Backward => {
                self.backward_data.lock().await.push(message);
            }
        }
    }

    pub async fn should_flush_forward(&self) -> bool {
        let locked = self.forward_data.lock().await;
        locked.iter().map(|m| m.datas.len()).sum::<usize>() >= FORWARD_FLUSH_THRESHOLD
    }

    pub async fn should_flush_backward(&self) -> bool {
        let locked = self.backward_data.lock().await;
        locked.iter().map(|m| m.datas.len()).sum::<usize>() >= BACKWARD_FLUSH_THRESHOLD
    }

    pub async fn drain_forward(&self) -> Vec<KlineMessage> {
        let mut locked = self.forward_data.lock().await;
        std::mem::take(&mut *locked)
    }

    pub async fn drain_backward(&self) -> Vec<KlineMessage> {
        let mut locked = self.backward_data.lock().await;
        std::mem::take(&mut *locked)
    }
}

#[async_trait]
pub trait FlushableBuffer: Send + Sync {
    async fn add(&self, msg: KlineMessage);
    async fn should_flush_forward(&self) -> bool;
    async fn should_flush_backward(&self) -> bool;
    async fn drain_forward(&self) -> Vec<KlineMessage>;
    async fn drain_backward(&self) -> Vec<KlineMessage>;
}

#[async_trait]
impl FlushableBuffer for KlineBuffer {
    async fn add(&self, msg: KlineMessage) {
        self.add(msg).await;
    }

    async fn should_flush_forward(&self) -> bool {
        self.should_flush_forward().await
    }

    async fn should_flush_backward(&self) -> bool {
        self.should_flush_backward().await
    }

    async fn drain_forward(&self) -> Vec<KlineMessage> {
        self.drain_forward().await
    }

    async fn drain_backward(&self) -> Vec<KlineMessage> {
        self.drain_backward().await
    }
}
