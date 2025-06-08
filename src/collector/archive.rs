use crate::collector::archive::types::ArchiveDirection;
use crate::domain::model::market_kline::NewOrUpdateMarketKline as MarketKlineInsertMySQL;
use crate::infra::external::binance::market::KlineSummary;
use crate::model::cex::kline::MarketKline as MarketKlineInsertCK;

pub mod dispatch_worker;
pub mod fetch;
pub mod flush;
pub mod kline_buffer;
pub mod sink;
mod types;

/// Trait：将 KlineMessage 转换为不同目标数据库的批量插入结构
pub trait IntoSinkRows<T> {
    fn into_sink_rows(&self) -> Vec<T>;
}

/// 表示一批 K线数据及其元信息（交易对、交易所、时间周期）
#[derive(Default, Clone, Debug)]
pub struct KlineMessage {
    /// K线数据摘要列表
    pub datas: Vec<KlineSummary>,

    /// 交易对名称，例如 "BTCUSDT"
    pub symbol: String,

    /// 交易所名称，例如 "binance"
    pub exchange: String,

    /// K线周期，例如 "1m", "5m", "1h"
    pub time_frame: String,

    /// 数据归档方向
    pub archive_direction: ArchiveDirection,
}

impl IntoSinkRows<MarketKlineInsertMySQL> for KlineMessage {
    fn into_sink_rows(&self) -> Vec<MarketKlineInsertMySQL> {
        self.datas
            .iter()
            .map(|k| {
                let base: MarketKlineInsertMySQL = (
                    k,
                    self.exchange.as_str(),
                    self.symbol.as_str(),
                    self.time_frame.as_str(),
                )
                    .into();
                base.into()
            })
            .collect()
    }
}

impl IntoSinkRows<MarketKlineInsertCK> for KlineMessage {
    fn into_sink_rows(&self) -> Vec<MarketKlineInsertCK> {
        self.datas
            .iter()
            .map(|k| {
                let base: MarketKlineInsertCK = (
                    k,
                    self.exchange.as_str(),
                    self.symbol.as_str(),
                    self.time_frame.as_str(),
                )
                    .into();
                base.into()
            })
            .collect()
    }
}
