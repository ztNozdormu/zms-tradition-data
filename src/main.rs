use zms_tradition_data::server;

#[tokio::main]
async fn main() {
    tracing::info!("Starting mini bot...");
    server::start().await;
}
