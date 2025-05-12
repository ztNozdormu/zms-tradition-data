mod aggregatoragg;
mod maintenance;
mod types;

use crate::model::TimeFrame;
use crate::trade_consumer::aggregatoragg::{CusAggregator, MultiTimeFrameAggregator};
use anyhow::Result;
use barter::barter_data::event::MarketEvent;
use barter::barter_data::exchange::binance::futures::BinanceFuturesUsd;
use barter::barter_data::streams::Streams;
use barter::barter_data::streams::reconnect::stream::ReconnectingStream;
use barter::barter_data::subscription::trade::{PublicTrade, PublicTrades};
use barter::barter_instrument::instrument::market_data::kind::MarketDataInstrumentKind;
use futures_util::StreamExt;
use std::collections::HashMap;
use tracing::warn;

pub async fn trade_driven_aggregation() -> Result<()> {
            // 初始化多周期聚合器 自定义聚合器
            let multi_aggregator =
                MultiTimeFrameAggregator::new(vec![TimeFrame::M1]);//, TimeFrame::M5, TimeFrame::M15

            let mut new_config: HashMap<String, Vec<TimeFrame>> = HashMap::new();
            new_config.insert(
                "btc".into(),
                vec![TimeFrame::M1],
            );

            multi_aggregator.merge_symbols_timeframes(new_config).await;

            // 初始化默认多周期聚合器
            // let multi_aggregator = MultiTimeFrameAggregator::new_with_defaults();

            let streams = Streams::<PublicTrades>::builder()
                .subscribe([(
                    BinanceFuturesUsd::default(),
                    "btc",
                    "usdt",
                    MarketDataInstrumentKind::Perpetual,
                    PublicTrades,
                )])
                .init()
                .await?;
            // Select and merge every exchange Stream using futures_util::stream::select_all
            // Note: use `Streams.select(ExchangeId)` to interact with individual exchange streams!
            let mut joined_stream = streams
                .select_all()
                .with_error_handler(|error| warn!(?error, "MarketStream generated error"));

            while let Some(event) = joined_stream.next().await {
                // info!("{event:?}");
                // Self(vec![Ok(MarketEvent {
                //     time_exchange: trade.time,
                //     _time_received: Utc::now(),
                //     exchange: exchange_id,
                //     instrument,
                //     kind: PublicTrade {
                //         id: trade.id.to_string(),
                //         price: trade.price,
                //         amount: trade.amount,
                //         side: trade.side,
                //     },
                // })])
                if let barter::barter_data::streams::reconnect::Event::Item(MarketEvent {
                                                                                time_exchange,
                                                                                time_received: _time_received,
                                                                                exchange,
                                                                                instrument,
                                                                                kind:
                                                                                PublicTrade {
                                                                                    id,
                                                                                    price,
                                                                                    amount,
                                                                                    side,
                                                                                },
                                                                            }) = event
                {
                    // 这里最好创建 aggregator 实例在循环外
                    multi_aggregator
                        .process_trade(
                    instrument.base.as_ref(),
                    exchange.as_str(),
                    time_exchange.timestamp_millis(),
                    &PublicTrade {
                        id,
                        price,
                        amount,
                        side,
                    },
                )
                .await;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_trade_driven_aggregation() {
        // 初始化 tracing 日志系统
        listen_tracing::setup_tracing();

        trade_driven_aggregation()
            .await
            .expect("Failed to run trade driven aggregation");
    }
}
