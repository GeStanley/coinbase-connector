use std::collections::BTreeMap;
use std::str::FromStr;

use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};

pub struct Order {
    pub side: String,
    pub time: DateTime<Utc>,
    pub price: BigDecimal,
    pub quantity: BigDecimal,
}

impl Order {
    pub fn new(side : &str, price: &str, quantity: &str, timestamp: DateTime<Utc>) -> Order {
        Order {
            side : side.to_string(),
            time: timestamp,
            price: BigDecimal::from_str(price).unwrap(),
            quantity: BigDecimal::from_str(quantity).unwrap(),
        }
    }
}

pub struct Book {
    pub product: String,
    pub bids: BTreeMap<BigDecimal, Order>,
    pub offers: BTreeMap<BigDecimal, Order>,
}

impl Book {
    pub fn new(product : String) -> Book {
        Book {
            product,
            bids: BTreeMap::new(),
            offers: BTreeMap::new(),
        }
    }
}

impl Book {
    pub fn insert_update(&mut self, side: &str, price: &str, quantity: &str, timestamp: DateTime<Utc>) {
        let order : Order = Order::new(side, price, quantity, timestamp);
        match side {
            "offer" => {
                self.offers.insert(order.price.clone(), order);
            },
            "bid" => {
                self.bids.insert(order.price.clone(), order);
            }
            _ => {
                println!("Received unrecognized order side.")
            }
        }
    }
}
