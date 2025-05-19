use bigdecimal::BigDecimal;
use chrono::{NaiveDate, NaiveDateTime};
use diesel::deserialize::Queryable;
use diesel::{Identifiable, Insertable};
use serde::{Deserialize, Serialize};

/// 加密货币分类表模型
#[derive(Debug, Queryable, Serialize, Deserialize, Identifiable, Clone)]
#[diesel(table_name = crate::schema::coin_categories)]
pub struct CoinCategory {
    /// 分类ID(如"world-liberty-financial-portfolio")
    pub id: String,

    /// 分类名称
    #[diesel(column_name = name)]
    pub name: String,

    /// 总市值(USD)
    pub market_cap: Option<BigDecimal>,

    /// 24小时市值变化百分比(%)
    pub market_cap_change_24h: Option<BigDecimal>,

    /// 分类详情内容
    pub content: Option<String>,

    /// 前三币种ID数组(JSON格式)
    pub top_3_coins_id: serde_json::Value,

    /// 前三币种信息(JSON格式)
    pub top_3_coins: serde_json::Value,

    /// 24小时交易量(USD)
    pub volume_24h: Option<BigDecimal>,

    /// 更新时间
    pub updated_at: Option<NaiveDateTime>,
}

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
    pub genesis_date: Option<NaiveDate>,

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

/// 加密货币市场排名信息模型
#[derive(Debug, Queryable, Serialize, Deserialize, Identifiable, Clone)]
#[diesel(table_name = crate::schema::coin_rank_info)]
pub struct CoinRankInfo {
    /// 币种唯一标识符(如"ethereum")
    pub id: String,

    /// 币种缩写(如"eth")
    #[diesel(column_name = symbol)]
    pub symbol: String,

    /// 币种全名
    #[diesel(column_name = name)]
    pub name: String,

    /// 币种图片URL
    pub image: Option<String>,

    /// 当前价格(USD)
    pub current_price: Option<BigDecimal>,

    /// 24小时价格变化(USD)
    pub price_change_24h: Option<BigDecimal>,

    /// 24小时价格变化百分比(%)
    pub price_change_percentage_24h: Option<BigDecimal>,

    /// 当前市值(USD)
    pub market_cap: Option<BigDecimal>,

    /// 市值排名
    pub market_cap_rank: Option<u32>,

    /// 24小时市值变化(USD)
    pub market_cap_change_24h: Option<BigDecimal>,

    /// 24小时市值变化百分比(%)
    pub market_cap_change_percentage_24h: Option<BigDecimal>,

    /// 完全稀释估值(USD)
    pub fully_diluted_valuation: Option<BigDecimal>,

    /// 24小时交易量(USD)
    pub total_volume: Option<BigDecimal>,

    /// 24小时内最高价(USD)
    pub high_24h: Option<BigDecimal>,

    /// 24小时内最低价(USD)
    pub low_24h: Option<BigDecimal>,

    /// 流通供应量
    pub circulating_supply: Option<BigDecimal>,

    /// 总供应量
    pub total_supply: Option<BigDecimal>,

    /// 最大供应量(可能为NULL)
    pub max_supply: Option<BigDecimal>,

    /// 历史最高价(USD)
    pub ath: Option<BigDecimal>,

    /// 距历史最高价变化百分比(%)
    pub ath_change_percentage: Option<BigDecimal>,

    /// 历史最高价日期
    pub ath_date: Option<NaiveDateTime>,

    /// 历史最低价(USD)
    pub atl: Option<BigDecimal>,

    /// 距历史最低价变化百分比(%)
    pub atl_change_percentage: Option<BigDecimal>,

    /// 历史最低价日期
    pub atl_date: Option<NaiveDateTime>,

    /// 投资回报率数据(JSON格式)
    pub roi: Option<serde_json::Value>,

    /// 最后更新时间
    pub last_updated: Option<NaiveDateTime>,
}

/// 用于创建新加密货币分类的模型
#[derive(Debug, Insertable, Serialize, Deserialize, Clone)]
#[diesel(table_name = crate::schema::coin_categories)]
pub struct NewCoinCategory {
    pub id: String,
    pub name: String,
    pub market_cap: Option<BigDecimal>,
    pub market_cap_change_24h: Option<BigDecimal>,
    pub content: Option<String>,
    pub top_3_coins_id: serde_json::Value,
    pub top_3_coins: serde_json::Value,
    pub volume_24h: Option<BigDecimal>,
    pub updated_at: Option<NaiveDateTime>,
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
    pub detail_platforms: Option<serde_json::Value>,
    pub block_time_in_minutes: Option<u32>,
    pub hashing_algorithm: Option<String>,
    pub categories: Option<serde_json::Value>,
    pub preview_listing: Option<bool>,
    pub public_notice: Option<String>,
    pub additional_notices: Option<serde_json::Value>,
    pub description: Option<serde_json::Value>,
    pub links: Option<serde_json::Value>,
    pub image: Option<serde_json::Value>,
    pub country_origin: Option<String>,
    pub genesis_date: Option<NaiveDate>,
    pub sentiment_votes_up_percentage: Option<BigDecimal>,
    pub sentiment_votes_down_percentage: Option<BigDecimal>,
    pub watchlist_portfolio_users: Option<u32>,
    pub market_cap_rank: Option<u32>,
    pub status_updates: Option<serde_json::Value>,
    pub last_updated: Option<NaiveDateTime>,
}

/// 用于创建新加密货币排名信息的模型
#[derive(Debug, Insertable, Serialize, Deserialize, Clone)]
#[diesel(table_name = crate::schema::coin_rank_info)]
pub struct NewCoinRankInfo {
    pub id: String,
    pub symbol: String,
    pub name: String,
    pub image: Option<String>,
    pub current_price: Option<BigDecimal>,
    pub price_change_24h: Option<BigDecimal>,
    pub price_change_percentage_24h: Option<BigDecimal>,
    pub market_cap: Option<BigDecimal>,
    pub market_cap_rank: Option<u32>,
    pub market_cap_change_24h: Option<BigDecimal>,
    pub market_cap_change_percentage_24h: Option<BigDecimal>,
    pub fully_diluted_valuation: Option<BigDecimal>,
    pub total_volume: Option<BigDecimal>,
    pub high_24h: Option<BigDecimal>,
    pub low_24h: Option<BigDecimal>,
    pub circulating_supply: Option<BigDecimal>,
    pub total_supply: Option<BigDecimal>,
    pub max_supply: Option<BigDecimal>,
    pub ath: Option<BigDecimal>,
    pub ath_change_percentage: Option<BigDecimal>,
    pub ath_date: Option<NaiveDateTime>,
    pub atl: Option<BigDecimal>,
    pub atl_change_percentage: Option<BigDecimal>,
    pub atl_date: Option<NaiveDateTime>,
    pub roi: Option<serde_json::Value>,
    pub last_updated: Option<NaiveDateTime>,
}

// 实现从 CoinRankInfo 到 NewCoinRankInfo 的转换
impl From<CoinRankInfo> for NewCoinRankInfo {
    fn from(info: CoinRankInfo) -> Self {
        NewCoinRankInfo {
            id: info.id,
            symbol: info.symbol,
            name: info.name,
            image: info.image,
            current_price: info.current_price,
            price_change_24h: info.price_change_24h,
            price_change_percentage_24h: info.price_change_percentage_24h,
            market_cap: info.market_cap,
            market_cap_rank: info.market_cap_rank,
            market_cap_change_24h: info.market_cap_change_24h,
            market_cap_change_percentage_24h: info.market_cap_change_percentage_24h,
            fully_diluted_valuation: info.fully_diluted_valuation,
            total_volume: info.total_volume,
            high_24h: info.high_24h,
            low_24h: info.low_24h,
            circulating_supply: info.circulating_supply,
            total_supply: info.total_supply,
            max_supply: info.max_supply,
            ath: info.ath,
            ath_change_percentage: info.ath_change_percentage,
            ath_date: info.ath_date,
            atl: info.atl,
            atl_change_percentage: info.atl_change_percentage,
            atl_date: info.atl_date,
            roi: info.roi,
            last_updated: info.last_updated,
        }
    }
}
