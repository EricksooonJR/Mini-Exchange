use axum::{
    extract::{State, WebSocketUpgrade},
    response::IntoResponse,
};
use axum::extract::ws::{WebSocket, Message};
use futures::{StreamExt, SinkExt};
use crate::AppState;

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, state.tx.subscribe()))
}

async fn handle_socket(
    mut socket: WebSocket,
    mut rx: tokio::sync::broadcast::Receiver<String>,
) {
    loop {
        tokio::select! {
            msg = rx.recv() => {
                if let Ok(msg) = msg {
                    if socket.send(Message::Text(msg)).await.is_err() {
                        break;
                    }
                }
            }
            Some(Ok(_)) = socket.next() => {}
        }
    }
}
