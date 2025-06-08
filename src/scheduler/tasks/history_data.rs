use crate::collector::archive::dispatch_worker::start_fair_task_scheduler;
use crate::domain::repository::market_symbol_repository::MarketSymbolRepository;
use crate::domain::service::market_symbol_service::MarketSymbolService;
use crate::global::get_mysql_pool;

/// 异步任务：系统启动维护交易所币种信息
pub async fn save_binance_symbol() -> Result<(), anyhow::Error> {
    let mut conn = get_mysql_pool().get()?;
    let repo = MarketSymbolRepository::new(&mut conn);
    let mut market_kline_service = MarketSymbolService { repo };
    market_kline_service.save_exchange_symbol_info().await
}

pub async fn exchange_history_data() -> Result<(), anyhow::Error> {
    start_fair_task_scheduler().await?;

    Ok(())
}
//
// pub async fn generate_tasks(
//     sender: mpsc::Sender<KlineMessage>,
//     symbols: &[&str],
//     intervals: &[&str],
// ) {
//     for symbol in symbols {
//         for interval in intervals {
//             let time_frame = TimeFrame::H1;
//             let kline_messages = historical_maintenance_process_tasks(symbol.to_string(),"binance".to_string(),Arc::new(time_frame)).await;
//             for kline_message in kline_messages {
//                 // ❸ 发送到 downstream
//                 if let Err(e) = sender.send(kline_message).await {
//                     eprintln!("failed to send KlineMessage: {e}");
//                 }
//             }
//         }
//     }
// }
