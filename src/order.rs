enum Side {
    Buy,
    Sell,
}
struct Order {
    id: u64,
    side: Side,
    price: u64,
    quantity: u64,
    timestamp: u64,
}

struct PriceLevel {}
