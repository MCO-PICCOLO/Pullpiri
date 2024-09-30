mod event;
mod grpc;
mod listener;
mod manager;

use manager::GatewayManager;

#[tokio::main]
async fn main() {
    let mut gateway_manager = GatewayManager::new();
    gateway_manager.run().await;
}
