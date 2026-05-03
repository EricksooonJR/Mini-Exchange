use axum::{routing::{post, get}, Json, Router};
use std::sync::{Arc, Mutex};

use crate::engine::OrderBook;
use crate::models::Order;
use crate::ws::{WsState, ws_handler};

pub fn create_routes(
    orderbook: Arc<Mutex<OrderBook>>,
    ws_state: WsState,
) -> Router {
    Router::new()
        .route("/order", post(create_order))
        .route("/ws", get(ws_handler))
        .with_state(ws_state)
        .with_state(orderbook)
}

async fn create_order(
    axum::extract::State(orderbook): axum::extract::State<Arc<Mutex<OrderBook>>>,
    axum::extract::State(ws_state): axum::extract::State<WsState>,
    Json(payload): Json<Order>,
) -> Json<String> {
    let mut ob = orderbook.lock().unwrap();

    ob.add_order(payload.clone());
    ob.match_orders();

    let message = format!("New order: {:?}", payload);
    let _ = ws_state.tx.send(message);

    Json(format!("Order {} processed", payload.id))
}
