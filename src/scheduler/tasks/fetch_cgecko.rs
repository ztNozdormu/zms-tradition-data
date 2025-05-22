use crate::domain::service::coin_category_service::save_categorys;
use crate::domain::service::coin_rank_info_service::save_coin_rank_info;

/// 异步任务：抓取 CoinGecko 排名并保存
pub async fn save_coin_rank_info_task() -> Result<(), anyhow::Error> {
    save_coin_rank_info().await
}

/// 异步任务：抓取 CoinGecko  Coin 板块分类
pub async fn save_categorys_task() -> Result<(), anyhow::Error> {
    save_categorys().await
}