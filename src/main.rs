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

#[tokio::main]
async fn main() {
    let wallet_repo:Arc<dyn WalletRepository> = Arc::new(MemoryWalletRepository::new());
    let app = Router::new()
    .route("/wallet/new", post(handler::create_wallet))
    .route("/internal/privateKey", post(handler::get_private_key))
    .with_state(wallet_repo);
    

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));  
    println!("service start on http://{}", addr);
    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

