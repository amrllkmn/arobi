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
        let mut bids: BTreeMap<u64, PriceLevel> = BTreeMap::new();
        let mut asks: BTreeMap<u64, PriceLevel> = BTreeMap::new();
        let order_map: HashMap<u64, (Side, u64)> = HashMap::new();

        let bid = Order {
            id: 1,
            side: Side::Bid,
            price: 100,
            quantity: 10,
            timestamp: 0,
        };

        let another_bid = Order {
            id: 2,
            side: Side::Bid,
            price: 95,
            quantity: 20,
            timestamp: 0,
        };

        let ask = Order {
            id: 3,
            side: Side::Ask,
            price: 90,
            quantity: 10,
            timestamp: 0,
        };

        let mut price_level_bids = PriceLevel {
            price: 100,
            orders: VecDeque::new(),
        };

        let mut second_bid_price_level = PriceLevel {
            price: 95,
            orders: VecDeque::new(),
        };

        let mut price_level_asks = PriceLevel {
            price: 90,
            orders: VecDeque::new(),
        };

        let incoming_ask = Order {
            id: 4,
            side: Side::Ask,
            price: 80,
            quantity: 30,
            timestamp: 0,
        };

        price_level_bids.orders.push_back(bid);
        second_bid_price_level.orders.push_back(another_bid);
        price_level_asks.orders.push_back(ask);

        bids.insert(100, price_level_bids);
        bids.insert(95, second_bid_price_level);
        asks.insert(90, price_level_asks);

        let mut order_book = OrderBook {
            bids,
            asks,
            order_map,
        };

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
        let mut bids: BTreeMap<u64, PriceLevel> = BTreeMap::new();
        let mut asks: BTreeMap<u64, PriceLevel> = BTreeMap::new();
        let order_map: HashMap<u64, (Side, u64)> = HashMap::new();

        let incoming_bid = Order {
            id: 3,
            side: Side::Bid,
            price: 100,
            quantity: 20,
            timestamp: 0,
        };

        let price_level_bids = PriceLevel {
            price: 100,
            orders: VecDeque::new(),
        };

        let mut price_level_asks = PriceLevel {
            price: 90,
            orders: VecDeque::new(),
        };

        let mut second_price_level_asks = PriceLevel {
            price: 85,
            orders: VecDeque::new(),
        };

        let ask = Order {
            id: 1,
            side: Side::Ask,
            price: 90,
            quantity: 10,
            timestamp: 0,
        };

        let second_ask = Order {
            id: 2,
            side: Side::Ask,
            price: 85,
            quantity: 10,
            timestamp: 0,
        };

        price_level_asks.orders.push_back(ask);
        second_price_level_asks.orders.push_back(second_ask);

        bids.insert(100, price_level_bids);
        asks.insert(90, price_level_asks);
        asks.insert(85, second_price_level_asks);

        let mut order_book = OrderBook {
            bids,
            asks,
            order_map,
        };

        let fills = order_book.add_limit_order(incoming_bid);
        assert_eq!(fills.len(), 2);
        assert_eq!(fills[0].price, 85);
        assert_eq!(fills[0].quantity, 10);
        assert_eq!(fills[0].maker_order_id, 2);
        assert_eq!(fills[1].price, 90);
        assert_eq!(fills[1].quantity, 10);
        assert_eq!(fills[1].maker_order_id, 1);
    }
}
