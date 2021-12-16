# dominos-coupons

Quickly scrape coupons from dominos.ca. Coupons are in the 1000-10000 range.

Find your store id in "Order Settings" under "My Store" as the "Store #".

## Usage
```sh
cargo build --release
STORE_ID=<your store id> ./target/release/dominos-coupons
```