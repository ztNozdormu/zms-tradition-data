#[macro_use]
mod macros;
mod db;
mod model;
mod server;
mod trade_consumer;
mod util;

mod global;

#[tokio::main]
async fn main() {
    tracing::info!("Starting mini bot...");
    server::start().await;
}
