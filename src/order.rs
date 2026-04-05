#![allow(dead_code)]
use std::collections::{BTreeMap, HashMap, VecDeque};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Side {
    Bid,
    Ask,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OrderType {
    Limit,
    Market,
    ImmediateOrCancel,
    FillOrKill,
    Cancel,
}

pub struct Order {
    pub id: u64,
    pub side: Side,
    pub price: u64,
    pub quantity: u64,
    pub timestamp: u64,
    pub order_type: OrderType,
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

impl Order {
    pub fn new_limit(id: u64, side: Side, price: u64, quantity: u64, timestamp: u64) -> Self {
        Self {
            id,
            side,
            order_type: OrderType::Limit,
            price,
            quantity,
            timestamp,
        }
    }
}

impl OrderBook {
    pub fn new() -> Self {
        Self {
            bids: BTreeMap::new(),
            asks: BTreeMap::new(),
            order_map: HashMap::new(),
        }
    }

    fn add_bid_order(&mut self, mut order: Order) -> Vec<Fill> {
        let mut fills: Vec<Fill> = Vec::new();
        let mut to_be_deleted: Vec<u64> = Vec::new();
        for (price, price_level) in self.asks.iter_mut() {
            if *price > order.price || order.quantity == 0 {
                break;
            } else {
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

        fills
    }
    fn add_ask_order(&mut self, mut order: Order) -> Vec<Fill> {
        let mut fills: Vec<Fill> = Vec::new();

        let mut to_be_deleted: Vec<u64> = Vec::new();

        for (price, price_level) in self.bids.iter_mut().rev() {
            if *price < order.price || order.quantity == 0 {
                break;
            } else {
                // ask is getting the best price
                while let Some(largest_bid) = price_level.orders.front_mut() {
                    if order.quantity == 0 {
                        break;
                    } else if largest_bid.quantity <= order.quantity {
                        fills.push(Fill {
                            price: *price,
                            quantity: largest_bid.quantity,
                            maker_order_id: largest_bid.id,
                            taker_order_id: order.id,
                        });
                        order.quantity -= largest_bid.quantity;
                        self.order_map.remove(&largest_bid.id);
                        price_level.orders.pop_front();

                        if price_level.orders.is_empty() {
                            to_be_deleted.push(*price);
                        }
                    } else {
                        fills.push(Fill {
                            price: *price,
                            quantity: order.quantity,
                            maker_order_id: largest_bid.id,
                            taker_order_id: order.id,
                        });
                        largest_bid.quantity -= order.quantity;
                        break;
                    }
                }
            }
        }

        for price in to_be_deleted {
            self.bids.remove(&price);
        }

        if order.quantity > 0 {
            self.order_map.insert(order.id, (order.side, order.price));
            self.asks
                .entry(order.price)
                .or_insert_with(|| PriceLevel {
                    price: order.price,
                    orders: VecDeque::new(),
                })
                .orders
                .push_back(order);
        }

        fills
    }
    pub fn add_limit_order(&mut self, order: Order) -> Vec<Fill> {
        // 1. check if bid or ask

        match order.side {
            // 2. match against opposite side:
            //      - find best price level on opposite side
            //      - while price crosses and incoming order has quantity remaining:
            //          - consume from front of VecDeque, create a fill
            //          - remove price level if empty
            // 3. if incoming order still has unfilled quantity:
            //      - add to order_map
            //      - insert into own side's price level (create if doesn't exist)
            Side::Bid => self.add_bid_order(order),
            Side::Ask => self.add_ask_order(order),
        }
    }
    pub fn add_market_order(&mut self, _side: Side, _qty: u64) -> Vec<Fill> {
        todo!()
    }
    pub fn cancel_order(&mut self, _order_id: u64) -> bool {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_limit_order_ask_create_fills() {
        let mut order_book = OrderBook::new();

        let bid = Order::new_limit(1, Side::Bid, 100, 10, 0);

        let another_bid = Order::new_limit(2, Side::Bid, 95, 20, 0);

        order_book.add_bid_order(bid);
        order_book.add_bid_order(another_bid);

        let incoming_ask = Order::new_limit(4, Side::Ask, 80, 30, 0);

        let fills = order_book.add_limit_order(incoming_ask);
        assert_eq!(fills.len(), 2);
        assert_eq!(fills[0].price, 100);
        assert_eq!(fills[0].quantity, 10);
        assert_eq!(fills[0].maker_order_id, 1);
        assert_eq!(fills[1].price, 95);
        assert_eq!(fills[1].quantity, 20);
        assert_eq!(fills[1].maker_order_id, 2);
    }

    #[test]
    fn test_add_limit_order_bid_create_fills() {
        let incoming_bid = Order::new_limit(3, Side::Bid, 100, 20, 0);

        let ask = Order::new_limit(1, Side::Ask, 90, 10, 0);

        let second_ask = Order::new_limit(2, Side::Ask, 85, 10, 0);

        let mut order_book = OrderBook::new();
        order_book.add_ask_order(ask);
        order_book.add_ask_order(second_ask);

        let fills = order_book.add_limit_order(incoming_bid);
        assert_eq!(fills.len(), 2);
        assert_eq!(fills[0].price, 85);
        assert_eq!(fills[0].quantity, 10);
        assert_eq!(fills[0].maker_order_id, 2);
        assert_eq!(fills[1].price, 90);
        assert_eq!(fills[1].quantity, 10);
        assert_eq!(fills[1].maker_order_id, 1);
    }

    #[test]
    fn test_add_limit_order_remainder_added_to_book() {
        let incoming_bid = Order::new_limit(3, Side::Bid, 100, 20, 0);

        let resting_ask = Order::new_limit(2, Side::Ask, 85, 10, 0);

        let mut order_book = OrderBook::new();
        order_book.add_ask_order(resting_ask);

        let fills = order_book.add_limit_order(incoming_bid);

        // remainder bid should be resting bid now
        let remainder = order_book.bids.get(&100);

        // remainder is in order map as well
        let bid_in_order_map = order_book.order_map.get(&3);
        assert!(bid_in_order_map.is_some());
        assert_eq!(bid_in_order_map.unwrap(), &(Side::Bid, 100));

        assert_eq!(fills.len(), 1);
        assert!(remainder.is_some());
        assert_eq!(remainder.unwrap().orders.len(), 1);
        assert_eq!(remainder.unwrap().orders[0].id, 3);
    }

    #[test]
    fn test_add_limit_order_clears_from_order_map() {
        let incoming_bid = Order::new_limit(3, Side::Bid, 100, 10, 0);

        let resting_ask = Order::new_limit(2, Side::Ask, 85, 10, 0);

        let mut order_book = OrderBook::new();

        order_book.add_ask_order(resting_ask);

        let fills = order_book.add_limit_order(incoming_bid);

        assert_eq!(fills.len(), 1);
        assert!(order_book.order_map.get(&2).is_none());
    }

    #[test]
    fn test_add_limit_order_unfilled_added_to_book() {
        let incoming_bid = Order::new_limit(2, Side::Bid, 100, 20, 0);

        let resting_ask = Order::new_limit(3, Side::Ask, 110, 10, 0);

        let mut order_book = OrderBook::new();
        order_book.add_ask_order(resting_ask);

        let fills = order_book.add_limit_order(incoming_bid);
        let new_resting = order_book.bids.get(&100);

        assert_eq!(fills.len(), 0);
        assert!(new_resting.is_some());
        assert_eq!(new_resting.unwrap().orders.len(), 1);
        assert_eq!(new_resting.unwrap().orders[0].id, 2); // inserted in fifo
    }

    #[test]
    fn test_add_limit_order_fills_in_fifo() {
        let incoming_bid = Order::new_limit(2, Side::Bid, 100, 20, 0);

        let first_resting_ask = Order::new_limit(1, Side::Ask, 85, 10, 0);

        let second_resting_ask = Order::new_limit(3, Side::Ask, 85, 10, 1);

        let mut order_book = OrderBook::new();
        order_book.add_ask_order(first_resting_ask);
        order_book.add_ask_order(second_resting_ask);

        let fills = order_book.add_limit_order(incoming_bid);

        assert_eq!(fills.len(), 2);
        assert_eq!(fills[0].price, 85);
        assert_eq!(fills[0].quantity, 10);
        assert_eq!(fills[0].maker_order_id, 1);
        assert_eq!(fills[1].price, 85);
        assert_eq!(fills[1].quantity, 10);
        assert_eq!(fills[1].maker_order_id, 3);
    }
}
