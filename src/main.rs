mod engine;
mod models;
mod routes;
mod ws;

use std::sync::{Arc, Mutex};
use tokio::net::TcpListener;
use tokio::sync::broadcast;

use engine::OrderBook;

// 👇 Estado global de la app
#[derive(Clone)]
pub struct AppState {
    pub orderbook: Arc<Mutex<OrderBook>>,
    pub tx: broadcast::Sender<String>,
}

#[tokio::main]
async fn main() {
    // OrderBook compartido
    let orderbook = Arc::new(Mutex::new(OrderBook::new()));

    // Canal para WebSockets
    let (tx, _) = broadcast::channel(100);

    // Estado global
    let app_state = AppState {
        orderbook,
        tx,
    };

    // Router
    let app = routes::create_routes(app_state);

    // 🔥 IMPORTANTE para Render
    let port = std::env::var("PORT").unwrap_or("3000".to_string());
    let addr = format!("0.0.0.0:{}", port);

    let listener = TcpListener::bind(&addr).await.unwrap();

    println!("🚀 Server running on http://{}", addr);

    axum::serve(listener, app).await.unwrap();
}
