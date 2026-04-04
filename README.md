# AROBI
**A**nother **R**ust **O**rder **B**ook **I**mplementation

## Why
I wanted to understand the kind of engineering work that happens in high frequency trading companies and why performant systems languages are typically used for the job. An order book is the core of every exchange, hence the project.

## Architecture
The order book uses a B-tree (`BTreeMap`) for both bids and asks to maintain the best ordering of prices for efficient access without scanning.

Within each price level, a `VecDeque` is used. This is to ensure that orders within a price-level are sorted by arrival time to ensure price-time priority.

The book also maintains a `HashMap` of key `order_id` and value `(side, price)`. This ensures that finding the price level for the given `order_id` is O(1) for cancellation, instead of scanning the entire bid side or ask side.

## Functionalities

### Add Limit Order
This uses price-time priority matching, where an incoming order arrives, it matches against best price (bids match against lowest asking price; asks match against highest bid price). Within each price level, the order is filled in FIFO order.
