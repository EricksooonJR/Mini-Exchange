use crate::models::{Order, OrderType};

#[derive(Default)]
pub struct OrderBook {
    pub buys: Vec<Order>,
    pub sells: Vec<Order>,
}

impl OrderBook {
    pub fn new() -> Self {
        Self {
            buys: vec![],
            sells: vec![],
        }
    }

    pub fn add_order(&mut self, order: Order) {
        match order.order_type {
            OrderType::Buy => self.buys.push(order),
            OrderType::Sell => self.sells.push(order),
        }
    }

    pub fn match_orders(&mut self) -> Vec<String> {
        let mut trades = vec![];

        self.buys.sort_by(|a, b| b.price.partial_cmp(&a.price).unwrap());
        self.sells.sort_by(|a, b| a.price.partial_cmp(&b.price).unwrap());

        while !self.buys.is_empty() && !self.sells.is_empty() {
            let buy = &self.buys[0];
            let sell = &self.sells[0];

            if buy.price >= sell.price {
                let qty = buy.quantity.min(sell.quantity);
                let trade_msg = format!(
                    "TRADE -> price: {}, qty: {}",
                    sell.price, qty
                );

                trades.push(trade_msg);

                self.buys.remove(0);
                self.sells.remove(0);
            } else {
                break;
            }
        }

        trades
    }
}
