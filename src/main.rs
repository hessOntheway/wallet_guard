mod service;
mod handler;

use std::net::SocketAddr;
use axum::{
    Router, routing::post, middleware
};
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;
use tracing_subscriber;


#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let app = Router::new()
    .route("/wallet/new", post(handler::create_wallet))
    .layer(TraceLayer::new_for_http());

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));  
    println!("service start on http://{}", addr);
    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

