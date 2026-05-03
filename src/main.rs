mod engine;
mod models;
mod routes;
mod ws;

use axum::Router;
use std::sync::{Arc, Mutex};
use tokio::net::TcpListener;
use tokio::sync::broadcast;

use engine::OrderBook;
use ws::WsState;

#[tokio::main]
async fn main() {
    let orderbook = Arc::new(Mutex::new(OrderBook::new()));

    // Canal para WebSocket
    let (tx, _) = broadcast::channel(100);

    let ws_state = WsState { tx };

    let app = routes::create_routes(orderbook, ws_state);

let port = std::env::var("PORT").unwrap_or("3000".to_string());

let listener = TcpListener::bind(format!("0.0.0.0:{}", port)).await.unwrap();

    println!("Server running on http://localhost:3000");

    axum::serve(listener, app).await.unwrap();
}
