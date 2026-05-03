mod engine;
mod models;
mod routes;

use axum::Router;
use std::sync::{Arc, Mutex};
use tokio::net::TcpListener;

use engine::OrderBook;

#[tokio::main]
async fn main() {
    let orderbook = Arc::new(Mutex::new(OrderBook::new()));

    let app = routes::create_routes(orderbook);

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("Server running on http://localhost:3000");

    axum::serve(listener, app).await.unwrap();
}
