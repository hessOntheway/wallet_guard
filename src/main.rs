mod service;
mod handler;
mod wallet;

use std::net::SocketAddr;
use axum::{
    Router, routing::post
};
use std::sync::Arc;
use tokio::net::TcpListener;
use wallet::{MemoryWalletRepository,WalletRepository};
use alloy_provider::ProviderBuilder;


#[derive(Clone)]
pub struct AppState {
    wallet_repo: Arc<dyn WalletRepository>,
    provider: Arc<dyn alloy_provider::Provider>,
}

#[tokio::main]
async fn main() {
    let wallet_repo: Arc<dyn WalletRepository> = Arc::new(MemoryWalletRepository::new());
    let eth_provider = ProviderBuilder::new()
        .connect_http("https://rpc.sepolia.org".parse().unwrap());

    let app_state = AppState {
        wallet_repo,
        provider: Arc::new(eth_provider),
    };
    
    let app = Router::new()
        .route("/wallet/new", post(handler::create_wallet))
        .route("/internal/privateKey", post(handler::get_private_key))
        .route("/tx/send", post(handler::send_tx))
        .with_state(app_state);
    

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));  
    println!("service start on http://{}", addr);
    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

