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
    pub fn new(side: &str, price: &str, quantity: &str, timestamp: DateTime<Utc>) -> Order {
        Order {
            side: side.to_string(),
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
    pub fn new(product: String) -> Book {
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
            }
            "bid" => {
                self.insert_bid(Order::new(side, price, quantity, timestamp));
            }
            _ => {
                println!("Received unrecognized order side.")
            }
        }
    }

    pub fn insert_bid(&mut self, order: Order) {
        if order.quantity == BigDecimal::from(0) && self.bids.contains_key(&order.price) {
            self.bids.remove(&order.price);
        } else if order.quantity != BigDecimal::from(0) {
            self.bids.insert(order.price.clone(), order);
        }
    }

    pub fn insert_offer(&mut self, order: Order) {
        if order.quantity == BigDecimal::from(0) && self.offers.contains_key(&order.price) {
            self.offers.remove(&order.price);
        } else if order.quantity != BigDecimal::from(0) {
            self.offers.insert(order.price.clone(), order);
        }
    }

    pub fn get_top_of_the_book(&mut self) -> Book {
        let mut top_of_the_book = Book::new(self.product.clone());
        match self.bids.last_entry() {
            None => {}
            Some(bid) => {
                top_of_the_book.insert_bid(bid.get().clone());
            }
        };
        match self.offers.first_entry() {
            None => {}
            Some(offer) => {
                top_of_the_book.insert_offer(offer.get().clone());
            }
        };
        top_of_the_book
    }
}
