use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OrderType {
    Buy,
    Sell,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Order {
    pub id: String,
    pub order_type: OrderType,
    pub price: f64,
    pub quantity: f64,
}

impl Order {
    pub fn new(order_type: OrderType, price: f64, quantity: f64) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            order_type,
            price,
            quantity,
        }
    }
}
