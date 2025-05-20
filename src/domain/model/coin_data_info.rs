use crate::common::serde_fun::{option_map_to_value, option_vec_to_value};
use crate::infra::external::cgecko::coin_data::CoinData;
use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use diesel::{Identifiable, Insertable, Queryable};
use serde::{Deserialize, Serialize};

/// 加密货币详细信息表模型
#[derive(Debug, Queryable, Serialize, Deserialize, Identifiable, Clone)]
#[diesel(table_name = crate::schema::coin_data_info)]
pub struct CoinDataInfo {
    /// 币种唯一标识符(如"bitcoin")
    pub id: String,

    /// 币种缩写(如"btc")
    #[diesel(column_name = symbol)]
    pub symbol: String,

    /// 币种全称
    #[diesel(column_name = name)]
    pub name: String,

    /// 网页URL后缀
    pub web_slug: Option<String>,

    /// 所属资产平台ID
    pub asset_platform_id: Option<String>,

    /// 支持的平台信息(JSON格式)
    pub platforms: Option<serde_json::Value>,

    /// 平台详细信息(JSON格式)
    pub detail_platforms: Option<serde_json::Value>,

    /// 出块时间(分钟)
    pub block_time_in_minutes: Option<u32>,

    /// 哈希算法(如"SHA-256")
    pub hashing_algorithm: Option<String>,

    /// 所属分类数组(JSON格式)
    pub categories: Option<serde_json::Value>,

    /// 是否预览上市
    pub preview_listing: Option<bool>,

    /// 公共通知
    pub public_notice: Option<String>,

    /// 附加通知数组(JSON格式)
    pub additional_notices: Option<serde_json::Value>,

    /// 多语言描述内容(JSON格式)
    pub description: Option<serde_json::Value>,

    /// 相关链接(JSON格式)
    pub links: Option<serde_json::Value>,

    /// 图片URL(JSON格式)
    pub image: Option<serde_json::Value>,

    /// 起源国家
    pub country_origin: Option<String>,

    /// 创世日期
    pub genesis_date: Option<NaiveDateTime>,

    /// 正面情绪投票百分比(%)
    pub sentiment_votes_up_percentage: Option<BigDecimal>,

    /// 负面情绪投票百分比(%)
    pub sentiment_votes_down_percentage: Option<BigDecimal>,

    /// 关注用户数
    pub watchlist_portfolio_users: Option<u32>,

    /// 市值排名
    pub market_cap_rank: Option<u32>,

    /// 状态更新数组(JSON格式)
    pub status_updates: Option<serde_json::Value>,

    /// 最后更新时间
    pub last_updated: Option<NaiveDateTime>,
}

/// 用于创建新加密货币详细信息的模型
#[derive(Debug, Insertable, Serialize, Deserialize, Clone)]
#[diesel(table_name = crate::schema::coin_data_info)]
pub struct NewCoinDataInfo {
    pub id: String,
    pub symbol: String,
    pub name: String,
    pub web_slug: Option<String>,
    pub asset_platform_id: Option<String>,
    pub platforms: Option<serde_json::Value>,
    pub block_time_in_minutes: Option<u32>,
    pub hashing_algorithm: Option<String>,
    pub categories: Option<serde_json::Value>,
    pub preview_listing: Option<bool>,
    pub public_notice: Option<String>,
    pub additional_notices: Option<serde_json::Value>,
    pub description: Option<serde_json::Value>,
    pub country_origin: Option<String>,
    pub genesis_date: Option<NaiveDateTime>,
    pub sentiment_votes_up_percentage: Option<BigDecimal>,
    pub sentiment_votes_down_percentage: Option<BigDecimal>,
    pub watchlist_portfolio_users: Option<u32>,
    pub market_cap_rank: Option<u32>,
    pub last_updated: Option<NaiveDateTime>,
}

// 实现从 CoinDataInfo 到 NewCoinDataInfo 的转换
impl From<CoinData> for NewCoinDataInfo {
    fn from(data: CoinData) -> Self {
        NewCoinDataInfo {
            id: data.id,
            symbol: data.symbol,
            name: data.name,
            web_slug: data.web_slug,
            asset_platform_id: data.asset_platform_id,
            platforms: option_map_to_value(data.platforms),
            block_time_in_minutes: data.block_time_in_minutes.map(|v| v),
            hashing_algorithm: data.hashing_algorithm,
            categories: Some(option_vec_to_value(data.categories)),
            preview_listing: Some(data.preview_listing),
            public_notice: data.public_notice,
            additional_notices: Some(option_vec_to_value(data.additional_notices)),
            description: option_map_to_value(data.description),
            country_origin: Some(data.country_origin),
            genesis_date: data.genesis_date,
            sentiment_votes_up_percentage: data.sentiment_votes_up_percentage,
            sentiment_votes_down_percentage: data.sentiment_votes_down_percentage,
            watchlist_portfolio_users: data.watchlist_portfolio_users,
            market_cap_rank: data.market_cap_rank,
            last_updated: data.last_updated,
        }
    }
}