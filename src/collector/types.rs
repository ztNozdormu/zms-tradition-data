use barter::barter_data::subscription::trade::PublicTrade;
use serde::{Deserialize, Serialize};
use trade_aggregation::candle_components::{Close, High, Low, NumTrades, Open, Volume};
use trade_aggregation::{CandleComponent, CandleComponentUpdate, ModularCandle, TakerTrade, Trade};

#[derive(Default, Clone)]
pub struct CusCandle {
    pub open: Open,
    pub high: High,
    pub low: Low,
    pub close: Close,
    pub volume: Volume,
    pub num_trades: NumTrades<u32>,
    pub time_range: FastTimeRange,
}

impl<T: TakerTrade> ModularCandle<T> for CusCandle {
    fn update(&mut self, trade: &T) {
        self.open.update(trade);
        self.high.update(trade);
        self.low.update(trade);
        self.close.update(trade);
        self.volume.update(trade);
        self.num_trades.update(trade);
        self.time_range.update(trade);
    }

    fn reset(&mut self) {
        self.open.reset();
        self.high.reset();
        self.low.reset();
        self.close.reset();
        self.volume.reset();
        self.num_trades.reset();
        self.time_range.reset();
    }
}

#[derive(Debug, Clone)]
pub struct FastTimeRange {
    pub open_time: i64,
    pub close_time: i64,
    initialized: bool,
}

impl Default for FastTimeRange {
    #[inline(always)]
    fn default() -> Self {
        Self {
            open_time: 0,
            close_time: 0,
            initialized: false,
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct TimeRangeValue {
    pub open: i64,
    pub close: i64,
}

impl TimeRangeValue {
    #[inline(always)]
    pub fn duration(&self) -> i64 {
        self.close - self.open
    }
}

impl CandleComponent<TimeRangeValue> for FastTimeRange {
    #[inline(always)]
    fn value(&self) -> TimeRangeValue {
        TimeRangeValue {
            open: self.open_time,
            close: self.close_time,
        }
    }

    #[inline(always)]
    fn reset(&mut self) {
        *self = Self::default();
    }
}

impl<T: TakerTrade> CandleComponentUpdate<T> for FastTimeRange {
    #[inline(always)]
    fn update(&mut self, trade: &T) {
        let ts = trade.timestamp();
        if !self.initialized {
            self.open_time = ts;
            self.initialized = true;
        }
        self.close_time = ts;
    }
}

// from barter PublicTrade to trade_aggregation Trade
//
// Self(vec![Ok(MarketEvent {
//     time_exchange: trade.time,
//     time_received: Utc::now(),
//     exchange: exchange_id,
//     instrument,
//     kind: PublicTrade {
//         id: trade.id.to_string(),
//         price: trade.price,
//         amount: trade.amount,
//         side: trade.side,
//     },
// })])

pub fn to_agg_trade(trade: &PublicTrade, timestamp: i64) -> Trade {
    Trade {
        timestamp,
        price: trade.price,
        size: trade.amount,
    }
}
