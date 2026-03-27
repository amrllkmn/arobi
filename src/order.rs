use std::collections::{BTreeMap, HashMap, VecDeque};

#[derive(Debug, Clone, Copy)]
pub enum Side {
    Bid,
    Ask,
}
pub struct Order {
    pub id: u64,
    pub side: Side,
    pub price: u64,
    pub quantity: u64,
    pub timestamp: u64,
}

struct PriceLevel {
    price: u64,
    orders: VecDeque<Order>,
}

pub struct Fill {
    pub price: u64,
    pub quantity: u64,
    pub maker_order_id: u64,
    pub taker_order_id: u64,
}

pub struct OrderBook {
    bids: BTreeMap<u64, PriceLevel>,      // highest price is best
    asks: BTreeMap<u64, PriceLevel>,      // lowest price is best
    order_map: HashMap<u64, (Side, u64)>, // key: order_id, value: (side, price)
}
