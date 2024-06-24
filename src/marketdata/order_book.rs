use std::collections::BTreeMap;
use std::str::FromStr;

use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
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

#[derive(Clone, Serialize, Deserialize)]
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

    pub fn insert_data(&mut self, side: &str, price: &str, quantity: &str, timestamp: DateTime<Utc>) {
        match side {
            "offer" => {
                self.insert_offer(Order::new(side, price, quantity, timestamp));
            },
            "bid" => {
                self.insert_bid(Order::new(side, price, quantity, timestamp));
            }
            _ => {
                println!("Received unrecognized order side.")
            }
        }
    }

    pub fn insert_bid(&mut self, order: Order) {
        self.bids.insert(order.price.clone(), order);
    }

    pub fn insert_offer(&mut self, order: Order) {
        self.offers.insert(order.price.clone(), order);
    }
}
