use std::collections::{BTreeMap, HashMap, VecDeque};

enum Side {
    Bid,
    Ask,
}
struct Order {
    id: u64,
    side: Side,
    price: u64,
    quantity: u64,
    timestamp: u64,
}

struct PriceLevel {
    price: u64,
    orders: VecDeque<Order>,
}

struct OrderBook {
    bids: BTreeMap<u64, PriceLevel>,      // highest price is best
    asks: BTreeMap<u64, PriceLevel>,      // lowest price is best
    order_map: HashMap<u64, (Side, u64)>, // key: order_id, value: (side, price)
}
