use crate::collector::archive::worker::start_worker_pool;
use crate::domain::repository::market_symbol_repository::MarketSymbolRepository;
use crate::domain::service::market_symbol_service::MarketSymbolService;
use crate::global::{get_flush_buffer, get_mysql_pool};
use barter::barter_xchange::exchange::binance::model::KlineSummary;
use tokio::sync::mpsc;

/// 异步任务：系统启动维护交易所币种信息
pub async fn save_binance_symbol() -> Result<(), anyhow::Error> {
    let mut conn = get_mysql_pool().get()?;
    let repo = MarketSymbolRepository::new(&mut conn);
    let mut market_kline_service = MarketSymbolService { repo };
    market_kline_service.save_exchange_symbol_info().await
}

pub async fn exchange_history_data() -> Result<(), anyhow::Error> {
    let (tx, rx) = mpsc::channel(1000);
    let buffer = get_flush_buffer();

    let symbols = vec!["BTCUSDT", "ETHUSDT", "SOLUSDT"];
    let intervals = vec!["1m", "5m", "1h"];

    // 启动 Worker Pool
    tokio::spawn(start_worker_pool(rx, 20));

    // 模拟任务投递
    generate_tasks(tx.clone(), &symbols, &intervals).await;

    Ok(())
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
}

pub async fn generate_tasks(
    sender: mpsc::Sender<KlineMessage>,
    symbols: &[&str],
    intervals: &[&str],
) {
    for symbol in symbols {
        for interval in intervals {
            // todo 模拟构建 RowData（请替换为你实际生成逻辑）
            // let summaries: Vec<KlineSummary> = fetch_binance_kline(symbol, interval).await;
            let summaries = Vec::<KlineSummary>::new(); //模拟构建

            // ❷ 封装为 KlineMessage，带上交易所名、周期
            let message = KlineMessage {
                datas: summaries,
                symbol: symbol.to_string(),
                exchange: "binance".to_string(),
                time_frame: interval.to_string(),
            };

            // ❸ 发送到 downstream
            if let Err(e) = sender.send(message).await {
                eprintln!("failed to send KlineMessage: {e}");
            }
        }
    }
}
