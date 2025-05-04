mod server;
mod util;
mod kv_store;
mod db;

#[tokio::main]
async fn main() {
  tracing::info!("Starting mini bot...");
  server::start().await;
}
