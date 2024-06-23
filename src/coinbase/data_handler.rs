use bytes::Bytes;
use chrono::{DateTime, Utc};
use log::info;

use crate::coinbase::api::websocket::{CoinbaseWebsocketMessage, Event, Update};
use crate::coinbase::api::websocket::MarketData::{snapshot, update};
use crate::marketdata::order_book::Book;
use crate::websocket::message_handler::MarketDataHandler;

pub struct CoinbaseDataHandler {
    pub order_book: Book,
}

impl MarketDataHandler for CoinbaseDataHandler {
    fn process_text(&mut self, bytes: Bytes) {
        let msg_utf8 = match std::str::from_utf8(&*bytes) {
            Ok(msg) => { msg }
            Err(_) => {
                panic!("Could not convert message to utf-8!");
            }
        };

        let message: CoinbaseWebsocketMessage = match serde_json::from_str(msg_utf8) {
            Ok(msg) => { msg }
            Err(_) => {
                info!("{}", msg_utf8);
                panic!("Could not deserialize message!");
            }
        };

        self.handle_coinbase_websocket_message(message);
    }
}

impl CoinbaseDataHandler {

    fn handle_coinbase_websocket_message(&mut self, message: CoinbaseWebsocketMessage) {
        info!("Received message sequence {}", message.sequence_num);
        for event in message.events.iter() {
            match event {
                Event::MarketData(data) => {
                    match data {
                        update { product_id, updates } => {
                            info!("This update for {:?} contains {:?} update!", product_id, updates.iter().size_hint());
                            match updates {
                                None => {}
                                Some(list) => { self.handle_update_list(list); }
                            }
                        }
                        snapshot { product_id, updates } => {
                            info!("This snapshot for {:?} contains {:?} update!", product_id, updates.iter().size_hint());
                            match updates {
                                None => {}
                                Some(list) => { self.handle_update_list(list); }
                            }
                        }
                    }
                }
                Event::Subscription(sub) => {
                    info!("Received a subscription confirmation!");
                }
            }
        }
    }

    fn handle_update_list(&mut self, updates: &Vec<Update>) {
        let mut update_iter = updates.iter();
        while let Some(update) = update_iter.next() {
            self.handle_update(update);
        }
    }

    fn handle_update(&mut self, update: &Update) {
        match update {
            Update::bid { event_time, price_level, new_quantity } => {
                self.order_book.insert_data("bid", price_level, new_quantity, event_time.parse::<DateTime<Utc>>().unwrap());
            }
            Update::offer { event_time, price_level, new_quantity } => {
                self.order_book.insert_data("offer", price_level, new_quantity, event_time.parse::<DateTime<Utc>>().unwrap());
            }
        }
    }
}