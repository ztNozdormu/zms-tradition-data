use crate::domain::service::coin_rank_info_service::save_coin_rank_info;

/// 异步任务：抓取 CoinGecko 排名并保存
pub async fn save_coin_rank_info_task() -> Result<(), anyhow::Error> {
    save_coin_rank_info().await
}