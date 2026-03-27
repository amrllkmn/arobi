use std::collections::{BTreeMap, HashMap, VecDeque};

#[derive(Debug, Clone, Copy, PartialEq)]
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

impl OrderBook {
    fn add_bid_order(&mut self, mut order: Order) -> Vec<Fill> {
        let mut fills: Vec<Fill> = Vec::new();
        let mut to_be_deleted: Vec<u64> = Vec::new();
        for (price, price_level) in self.asks.iter_mut() {
            if price <= &order.price && order.quantity > 0 {
                // the bid is getting the cheapest deal
                // peek from deque remove as much quantity
                // pop if quantity is zero
                while let Some(lowest_ask) = price_level.orders.front_mut() {
                    if order.quantity == 0 {
                        break;
                    } else if lowest_ask.quantity <= order.quantity {
                        fills.push(Fill {
                            price: *price,
                            quantity: lowest_ask.quantity,
                            maker_order_id: lowest_ask.id,
                            taker_order_id: order.id,
                        });
                        order.quantity -= lowest_ask.quantity;
                        self.order_map.remove(&lowest_ask.id);
                        price_level.orders.pop_front();

                        if price_level.orders.is_empty() {
                            to_be_deleted.push(*price);
                        }
                    } else {
                        fills.push(Fill {
                            price: *price,
                            quantity: order.quantity,
                            maker_order_id: lowest_ask.id,
                            taker_order_id: order.id,
                        });
                        lowest_ask.quantity -= order.quantity;
                        break;
                    }
                }
            }
        }

        for price in to_be_deleted {
            self.asks.remove(&price);
        }

        if order.quantity > 0 {
            self.order_map.insert(order.id, (order.side, order.price));
            self.bids
                .entry(order.price)
                .or_insert_with(|| PriceLevel {
                    price: order.price,
                    orders: VecDeque::new(),
                })
                .orders
                .push_back(order);
        }

        return fills;
    }
    fn add_ask_order(&mut self, _order: Order) -> Vec<Fill> {
        todo!()
    }
    pub fn add_limit_order(&mut self, order: Order) -> Vec<Fill> {
        // 1. check if bid or ask

        match order.side {
            Side::Bid => self.add_bid_order(order),
            Side::Ask => self.add_ask_order(order),
        }
        // 2. match against opposite side:
        //      - find best price level on opposite side
        //      - while price crosses and incoming order has quantity remaining:
        //          - consume from front of VecDeque, create a fill
        //          - remove price level if empty
        // 3. if incoming order still has unfilled quantity:
        //      - add to order_map
        //      - insert into own side's price level (create if doesn't exist)
    }
    pub fn add_market_order(&mut self, _side: Side, _qty: u64) -> Vec<Fill> {
        todo!()
    }
    pub fn cancel_order(&mut self, _order_id: u64) -> bool {
        todo!()
    }
}
