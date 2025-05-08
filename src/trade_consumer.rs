mod aggregatoragg;
mod types;

use crate::db::ckdb::ClickhouseDb;
use anyhow::Result;
use barter::barter_data::exchange::binance::futures::BinanceFuturesUsd;
use barter::barter_data::streams::Streams;
use barter::barter_data::streams::reconnect::stream::ReconnectingStream;
use barter::barter_data::subscription::trade::PublicTrades;
use barter::barter_instrument::instrument::market_data::kind::MarketDataInstrumentKind;
use futures_util::StreamExt;
use std::sync::Arc;
use tracing::{info, warn};

pub async fn trade_driven_aggregation(_db: Arc<ClickhouseDb>) -> Result<()> {
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
        // todo trade 数据事件 触发聚合器调用
        info!("{event:?}");
    }
    Ok(())
}
