use crate::common::serde_fun::option_obj_to_value;
use crate::domain::model::SortOrder;
use crate::infra::external::cgecko::coin_rank::CoinRank;
use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use diesel::{AsChangeset, Identifiable, Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};

/// 加密货币市场排名信息模型
#[derive(Debug, Queryable, Selectable, Serialize, Deserialize, Identifiable, Clone)]
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

/// 用于创建新加密货币排名信息的模型
#[derive(Debug, Identifiable, Insertable, AsChangeset, Serialize, Deserialize, Clone)]
#[diesel(table_name = crate::schema::coin_rank_info)]
pub struct NewOrUpdateCoinRankInfo {
    pub id: String,
    pub symbol: String,
    pub name: String,
    pub image: String,
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

// 实现从 CoinRank 到 NewOrUpdateCoinRankInfo 的转换
impl From<CoinRank> for NewOrUpdateCoinRankInfo {
    fn from(info: CoinRank) -> Self {
        NewOrUpdateCoinRankInfo {
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
            roi: option_obj_to_value(info.roi),
            last_updated: info.last_updated,
        }
    }
}

#[derive(Debug, Clone)]
pub struct CoinRankInfoFilter {
    pub symbol: Option<String>,
    pub symbol_like: Option<String>,
    pub min_rank: Option<u32>,
    pub max_rank: Option<u32>,
    pub sort_by_rank: Option<SortOrder>,
    pub page: Option<usize>,
    pub page_size: Option<usize>,
}
