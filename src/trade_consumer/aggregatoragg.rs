use crate::db::ckdb::Database;
use crate::model::TimeFrame;
use crate::model::cex::kline::MarketKline;
use crate::trade_consumer::types::{CusCandle, to_agg_trade};
use barter::barter_data::subscription::trade::PublicTrade;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use trade_aggregation::candle_components::*;
use trade_aggregation::{
    Aggregator as TradeAggregator, GenericAggregator, TimeRule, TimestampResolution, Trade,
};

/// 聚合器接口定义
#[async_trait::async_trait]
pub trait CusAggregator {
    async fn process_trade(
        &self,
        symbol: &str,
        exchange: &str,
        timestamp: i64,
        trade: &PublicTrade,
    ) -> Vec<MarketKline>;
}

/// 多周期K线聚合器实现
pub struct MultiTimeFrameAggregator {
    timeframes: Vec<TimeFrame>,
    aggregators:
        Arc<RwLock<HashMap<(String, TimeFrame), GenericAggregator<CusCandle, TimeRule, Trade>>>>,
}

impl MultiTimeFrameAggregator {
    pub fn new(timeframes: Vec<TimeFrame>) -> Self {
        Self {
            timeframes,
            aggregators: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    fn to_market_kline(
        &self,
        exchange: &str,
        symbol: &str,
        period: &str,
        candle: &CusCandle,
    ) -> MarketKline {
        MarketKline {
            exchange: Arc::from(exchange),
            symbol: Arc::from(symbol),
            period: Arc::from(period),
            open_time: candle.time_range.open_time,
            close_time: candle.time_range.close_time,
            open: candle.open.value(),
            high: candle.high.value(),
            low: candle.low.value(),
            close: candle.close.value(),
            volume: candle.volume.value(),
            quote_asset_volume: 0.0,
            number_of_trades: candle.num_trades.value() as u64,
            taker_buy_base_asset_volume: 0.0,
            taker_buy_quote_asset_volume: 0.0,
        }
    }
}

#[async_trait::async_trait]
impl CusAggregator for MultiTimeFrameAggregator {
    async fn process_trade(
        &self,
        symbol: &str,
        exchange: &str,
        timestamp: i64,
        trade: &PublicTrade,
    ) -> Vec<MarketKline> {
        let mut results = Vec::new();
        let mut aggrs = self.aggregators.write().await;

        for tf in &self.timeframes {
            let key = (symbol.to_string(), tf.clone());

            let aggr = aggrs.entry(key.clone()).or_insert_with(|| {
                GenericAggregator::<CusCandle, TimeRule, Trade>::new(
                    TimeRule::new(tf.to_period(), TimestampResolution::Millisecond),
                    false,
                )
            });

            let trade = to_agg_trade(trade, timestamp);
            if let Some(candle) = aggr.update(&trade) {
                results.push(self.to_market_kline(
                    symbol,
                    exchange,
                    tf.to_str(),
                    &candle,
                ));
            }
        }

        results
    }
}
