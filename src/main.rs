mod server;
mod util;
mod kv_store;
mod db;
mod model;

#[tokio::main]
async fn main() {
  tracing::info!("Starting mini bot...");
  server::start().await;
}
