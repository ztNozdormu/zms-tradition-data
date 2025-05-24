use crate::domain::repository::coin_category_repository::CoinCategoryRepository;
use crate::domain::repository::coin_data_info_repository::CoinDataInfoRepository;
use crate::domain::repository::coin_rank_info_repository::CoinRankInfoRepository;
use crate::domain::service::coin_category_service::CoinCategoryService;
use crate::domain::service::coin_data_info_service::CoinDataInfoService;
use crate::domain::service::coin_rank_info_service::CoinRankInfoService;
use crate::global::get_mysql_pool;

/// 异步任务：抓取 CoinGecko 排名并保存
pub async fn save_coin_rank_info_task() -> Result<(), anyhow::Error> {
    let mut conn = get_mysql_pool().get()?;
    let repo = CoinRankInfoRepository::new(&mut conn);
    let mut coin_rank_info_service = CoinRankInfoService { repo };
    coin_rank_info_service.save_coin_rank_info().await
}

/// 异步任务：抓取 CoinGecko  Coin 板块分类
pub async fn save_categorys_task() -> Result<(), anyhow::Error> {
    let mut conn = get_mysql_pool().get()?;
    let repo = CoinCategoryRepository::new(&mut conn);
    let mut coin_category_service = CoinCategoryService { repo };
    coin_category_service.save_categorys().await
}

/// 异步任务：抓取 Coin_data_info 信息包含所属板块
pub async fn save_coin_data_info_task() -> Result<(), anyhow::Error> {
    // todo 获取所有币种循环获取
    let coin_id = "bitcoin";
    let mut conn = get_mysql_pool().get()?;
    let repo = CoinDataInfoRepository::new(&mut conn);
    let mut coin_data_info_service = CoinDataInfoService { repo };
    coin_data_info_service.save_coin_data_info(coin_id).await
}
