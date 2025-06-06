use crate::collector::maintenance::historical_maintenance_process;
use crate::collector::types::{to_agg_trade, CusCandle};
use crate::global::{get_ck_db, get_flush_controller};
use crate::model::cex::kline::MarketKline;
use crate::model::{TimeFrame, DEFAULT_TIMEFRAMES};
use async_trait::async_trait;
use barter::barter_data::subscription::trade::PublicTrade;
use std::collections::HashSet;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;
use tracing::{error, info};
use trade_aggregation::{
    Aggregator, CandleComponent, GenericAggregator, TimeRule, TimestampResolution, Trade,
};

/// 多周期K线聚合器实现
#[async_trait]
pub trait CusAggregator {
    async fn process_trade(
        &self,
        symbol: &str,
        exchange: &str,
        timestamp: i64,
        trade: &PublicTrade,
    );
}

pub struct MultiTimeFrameAggregator {
    timeframes: Vec<TimeFrame>,
    symbol_timeframes: Arc<RwLock<HashMap<String, Vec<TimeFrame>>>>,
    aggregators:
        Arc<RwLock<HashMap<(String, TimeFrame), GenericAggregator<CusCandle, TimeRule, Trade>>>>,
}

impl MultiTimeFrameAggregator {
    pub fn new(timeframes: Vec<TimeFrame>) -> Self {
        Self {
            timeframes,
            aggregators: Arc::new(RwLock::new(HashMap::new())),
            symbol_timeframes: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn new_with_defaults() -> Self {
        Self {
            timeframes: DEFAULT_TIMEFRAMES.to_vec(),
            aggregators: Arc::new(RwLock::new(HashMap::new())),
            symbol_timeframes: Arc::new(RwLock::new(Self::default_symbol_timeframes())),
        }
    }

    fn default_symbol_timeframes() -> HashMap<String, Vec<TimeFrame>> {
        let tf = DEFAULT_TIMEFRAMES.to_vec();
        HashMap::from([
            ("btc".into(), tf.clone()),
            ("eth".into(), tf.clone()),
            ("bnb".into(), tf.clone()),
            ("sol".into(), tf.clone()),
        ])
    }

    /// 合并符号和时间周期配置：去重并更新已有配置
    pub async fn merge_symbols_timeframes<I, S>(&self, configs: I)
    where
        I: IntoIterator<Item = (S, Vec<TimeFrame>)>,
        S: Into<String>,
    {
        let mut map = self.symbol_timeframes.write().await;

        for (symbol, new_tfs) in configs {
            let entry = map.entry(symbol.into()).or_insert_with(Vec::new);
            let mut set: HashSet<TimeFrame> = entry.drain(..).collect();
            set.extend(new_tfs);
            let mut combined: Vec<_> = set.into_iter().collect();
            combined.sort_unstable();
            *entry = combined;
        }
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
            exchange: exchange.to_string(),
            symbol: symbol.to_string(),
            period: period.to_string(),
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
            let mut symbol_map = self.symbol_timeframes.write().await;
            symbol_map.remove(symbol);
        }

        {
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
    ) {
        let trade = to_agg_trade(trade, timestamp);

        // 将 timeframes 明确 clone 出来，避免生命周期问题
        let timeframes = {
            let map = self.symbol_timeframes.read().await;
            map.get(symbol)
                .cloned()
                .unwrap_or_else(|| self.timeframes.clone())
        };

        let mut aggrs = self.aggregators.write().await;

        // let ck_db = get_ck_db();

        for tf in timeframes.iter() {
            let aggr = self.get_or_create_aggregator(&mut aggrs, symbol, tf);

            if let Some(candle) = aggr.update(&trade) {
                let market_kline = self.to_market_kline(exchange, symbol, tf.to_str(), &candle);

                info!(
                    "Generated candle at tf {:?} for symbol {}: {:?}",
                    tf, symbol, market_kline
                );

                // 确保插入操作完成 comment
                // if let Err(e) = get_flush_controller().push(exchange,symbol,tf.to_str(),SEGMENT_RECENT,&market_kline).await {
                //     error!("Insert market_kline failed: {:?}", e);
                // } else {
                let symbol_clone = symbol.to_string();
                let exchange_clone = exchange.to_string();
                let tf_clone = tf.clone();
                let close_time = trade.timestamp;
                // 插入成功后，执行历史数据维护
                tokio::spawn(async move {
                    historical_maintenance_process(
                        symbol_clone,
                        exchange_clone,
                        close_time,
                        Arc::new(tf_clone),
                    )
                    .await;
                });
                // }
            }
        }
    }
}
