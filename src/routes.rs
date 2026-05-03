use axum::{routing::post, Json, Router};
use std::sync::{Arc, Mutex};

use crate::engine::OrderBook;
use crate::models::{Order, OrderType};

pub fn create_routes(orderbook: Arc<Mutex<OrderBook>>) -> Router {
    Router::new()
        .route("/order", post(create_order))
        .with_state(orderbook)
}

async fn create_order(
    axum::extract::State(orderbook): axum::extract::State<Arc<Mutex<OrderBook>>>,
    Json(payload): Json<Order>,
) -> Json<String> {
    let mut ob = orderbook.lock().unwrap();

    ob.add_order(payload.clone());
    ob.match_orders();

    Json(format!("Order {} processed", payload.id))
}
