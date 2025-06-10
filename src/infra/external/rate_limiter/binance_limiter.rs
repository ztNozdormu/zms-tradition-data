use governor::{
    clock::DefaultClock,
    state::{InMemoryState, NotKeyed},
    Jitter, Quota, RateLimiter,
};
use std::{num::NonZeroU32, sync::Arc, time::Duration};

type InnerLimiter = RateLimiter<NotKeyed, InMemoryState, DefaultClock>;

#[derive(Clone)]
pub struct BinanceLimiter {
    limiter: Arc<InnerLimiter>,
}

impl BinanceLimiter {
    /// 创建新的 BinanceLimiter 实例，2400次/分钟
    pub fn new() -> Self {
        let quota = Quota::per_minute(NonZeroU32::new(2400).unwrap());
        let limiter = RateLimiter::direct(quota);
        Self {
            limiter: Arc::new(limiter),
        }
    }

    /// 异步获取令牌，等待可用（含抖动）
    pub async fn acquire(&self) {
        let jitter = Jitter::up_to(Duration::from_millis(30));
        self.limiter.until_ready_with_jitter(jitter).await;
    }

    /// 非阻塞尝试获取令牌
    pub fn try_acquire(&self) -> bool {
        self.limiter.check().is_ok()
    }
}
