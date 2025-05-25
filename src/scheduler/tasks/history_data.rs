use crate::domain::repository::market_kline_repository::MarketKlineRepository;
use crate::domain::service::market_kline_service::MarketKlineService;
use crate::global::get_mysql_pool;

/// 异步任务：定时维护交易所不同币种不同周期的历史k线数据
pub async fn exchange_history_data() -> Result<(), anyhow::Error> {
    let mut conn = get_mysql_pool().get()?;
    let repo = MarketKlineRepository::new(&mut conn);
    let mut market_kline_service = MarketKlineService { repo };
    market_kline_service.exchange_history_data().await
}
