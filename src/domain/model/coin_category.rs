use crate::common::serde_fun::option_vec_to_value;
use crate::domain::model::coin_rank_info::{CoinRankInfo, NewCoinRankInfo};
use crate::infra::external::cgecko::coin_categories::CoinCategories;
use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use diesel::{Identifiable, Insertable, Queryable};
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

// 实现从 CoinCategories 到 NewCoinCategory 的转换
impl From<CoinCategories> for NewCoinCategory {
    fn from(info: CoinCategories) -> Self {
        NewCoinCategory {
            id: info.id.clone(),
            name: info.name.clone(),
            market_cap: info.market_cap.clone(),
            market_cap_change_24h: info.market_cap_change_24h.clone(),
            content: info.content.clone(),
            top_3_coins_id: option_vec_to_value(info.top_3_coins_id),
            top_3_coins: option_vec_to_value(info.top_3_coins),
            volume_24h: info.volume_24h.clone(),
            updated_at: info.updated_at.clone(),
        }
    }
}
