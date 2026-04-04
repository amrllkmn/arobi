# AROBI
**A**nother **R**ust **O**rder **B**ook **I**mplementation

## Why
I wanted to understand the kind of engineering work that happens in high frequency trading companies and why performant systems languages are typically used for the job. An order book is the core of every exchange, hence the project.

## Functionalities

### Add Limit Order
This uses price-time priority, where an incoming order arrives, it matches against best price (lowest price for incoming bids, highest price for incoming ask), and fills it in FIFO manner.
