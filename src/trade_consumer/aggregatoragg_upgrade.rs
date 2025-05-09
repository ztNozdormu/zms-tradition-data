use std::{
    borrow::Cow,
    collections::HashMap,
    sync::Arc,
};

use async_trait::async_trait;
use barter::barter_data::subscription::trade::PublicTrade;
use tokio::sync::RwLock;
use trade_aggregation::{Aggregator, CandleComponent, GenericAggregator, TimeRule, TimestampResolution, Trade};
use crate::model::cex::kline::MarketKline;
use crate::model::TimeFrame;
use crate::trade_consumer::types::{to_agg_trade, CusCandle};

/// 多周期K线聚合器实现
#[async_trait]
pub trait CusAggregator {
    async fn process_trade(
        &self,
        symbol: &str,
        exchange: &str,
        timestamp: i64,
        trade: &PublicTrade,
    ) -> Vec<MarketKline>;
}

pub struct MultiTimeFrameAggregator {
    default_timeframes: Vec<TimeFrame>,
    symbol_timeframes: Arc<RwLock<HashMap<String, Vec<TimeFrame>>>>,
    aggregators: Arc<RwLock<HashMap<(String, TimeFrame), GenericAggregator<CusCandle, TimeRule, Trade>>>>,
}

impl MultiTimeFrameAggregator {
    pub fn new(default_timeframes: Vec<TimeFrame>) -> Self {
        Self {
            default_timeframes,
            symbol_timeframes: Arc::new(RwLock::new(HashMap::new())),
            aggregators: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn set_symbol_timeframes(&self, symbol: &str, timeframes: Vec<TimeFrame>) {
        let mut symbol_map = self.symbol_timeframes.write().await;
        symbol_map.insert(symbol.to_string(), timeframes);
    }

    fn get_or_create_aggregator<'a>(
        &'a self,
        aggrs: &'a mut HashMap<(String, TimeFrame), GenericAggregator<CusCandle, TimeRule, Trade>>,
        symbol: &str,
        tf: &TimeFrame,
    ) -> &'a mut GenericAggregator<CusCandle, TimeRule, Trade> {
        let key = (symbol.to_string(), tf.clone());
        aggrs.entry(key).or_insert_with(|| {
            GenericAggregator::new(
                TimeRule::new(tf.to_period(), TimestampResolution::Millisecond),
                false,
            )
        })
    }

    fn to_market_kline(
        &self,
        exchange: &str,
        symbol: &str,
        period: &str,
        candle: &CusCandle,
    ) -> MarketKline {
        MarketKline {
            exchange: Cow::Borrowed(exchange).into(),
            symbol: Cow::Borrowed(symbol).into(),
            period: Cow::Borrowed(period).into(),
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

    pub async fn remove_symbol(&self, symbol: &str) {
        {
            // 移除 symbol 对应的自定义时间周期配置
            let mut symbol_map = self.symbol_timeframes.write().await;
            symbol_map.remove(symbol);
        }

        {
            // 移除 symbol 对应的所有 aggregators
            let mut aggrs = self.aggregators.write().await;
            aggrs.retain(|(s, _), _| s != symbol);
        }
    }
}

#[async_trait]
impl CusAggregator for MultiTimeFrameAggregator {
    async fn process_trade(
        &self,
        symbol: &str,
        exchange: &str,
        timestamp: i64,
        trade: &PublicTrade,
    ) -> Vec<MarketKline> {
        let mut results = Vec::new();
        let trade = to_agg_trade(trade, timestamp);

        let timeframes = {
            let map = self.symbol_timeframes.read().await;
            map.get(symbol).cloned().unwrap_or_else(|| self.default_timeframes.clone())
        };

        let mut aggrs = self.aggregators.write().await;

        for tf in timeframes {
            let aggr = self.get_or_create_aggregator(&mut aggrs, symbol, &tf);
            if let Some(candle) = aggr.update(&trade) {
                results.push(self.to_market_kline(exchange, symbol, tf.to_str(), &candle));
            }
        }

        results
    }
}