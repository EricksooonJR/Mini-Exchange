use axum::{
    routing::{post, get},
    Json, Router, extract::State
};

use crate::{models::Order, ws::ws_handler, AppState};

pub fn create_routes(state: AppState) -> Router {
    Router::new()
        .route("/order", post(create_order))
        .route("/ws", get(ws_handler))
        .with_state(state)
}

async fn create_order(
    State(state): State<AppState>,
    Json(payload): Json<Order>,
) -> Json<String> {

    let mut ob = state.orderbook.lock().unwrap();

    ob.add_order(payload.clone());

    // 🔥 Ejecuta matching
    let trades = ob.match_orders();

    // 🔥 Envía trades por WebSocket
    for trade in trades {
        let _ = state.tx.send(trade);
    }

    Json(format!("Order {} processed", payload.id))
}
