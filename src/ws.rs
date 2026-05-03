use axum::{
    extract::ws::{WebSocket, WebSocketUpgrade, Message},
    extract::State,
    response::IntoResponse,
};
use futures::{StreamExt, SinkExt};
use tokio::sync::broadcast::{Sender, Receiver};

#[derive(Clone)]
pub struct WsState {
    pub tx: Sender<String>,
}

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<WsState>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, state.tx.subscribe()))
}

async fn handle_socket(mut socket: WebSocket, mut rx: Receiver<String>) {
    println!("Cliente conectado");

    loop {
        tokio::select! {
            // Recibe mensajes del servidor
            msg = rx.recv() => {
                if let Ok(msg) = msg {
                    if socket.send(Message::Text(msg)).await.is_err() {
                        break;
                    }
                }
            }

            // Recibe mensajes del cliente (opcional)
            Some(Ok(Message::Text(text))) = socket.next() => {
                println!("Cliente dice: {}", text);
            }
        }
    }

    println!("Cliente desconectado");
}
