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
    /// 创建新的 BinanceLimiter 实例，默认配额 2400 次/分钟（即权重总和）
    pub fn new() -> Self {
        let quota = Quota::per_minute(NonZeroU32::new(2400).unwrap());
        let limiter = RateLimiter::direct(quota);
        Self {
            limiter: Arc::new(limiter),
        }
    }

    /// 根据 limit 动态计算请求权重
    fn calc_weight(limit: u32) -> NonZeroU32 {
        match limit {
            1..=99 => NonZeroU32::new(1).unwrap(),
            100..=499 => NonZeroU32::new(2).unwrap(),
            500..=1000 => NonZeroU32::new(5).unwrap(),
            _ => panic!("Binance API limit > 1000 is not supported"),
        }
    }

    /// 异步等待获取权重对应的令牌（含抖动）
    pub async fn acquire_with_limit(&self, limit: u32) {
        let weight = Self::calc_weight(limit);
        let jitter = Jitter::up_to(Duration::from_millis(30));
        let _ = self.limiter.until_n_ready_with_jitter(weight, jitter).await;
    }

    /// 非阻塞尝试获取权重对应的令牌，返回是否成功
    pub fn try_acquire_with_limit(&self, limit: u32) -> bool {
        let weight = match limit {
            1..=1000 => Self::calc_weight(limit),
            _ => return false,
        };
        self.limiter.check_n(weight).is_ok()
    }

    /// 传统的单次请求令牌异步等待（相当于权重1）
    pub async fn acquire(&self) {
        self.acquire_with_limit(1).await;
    }

    /// 传统的单次请求非阻塞尝试（相当于权重1）
    pub fn try_acquire(&self) -> bool {
        self.try_acquire_with_limit(1)
    }
}
