use std::str::Utf8Error;
use bytes::Bytes;
use crate::coinbase::api::websocket::{CoinbaseWebsocketMessage, Event, MarketData};
use crate::coinbase::api::websocket::MarketData::{snapshot, update};
use crate::websocket::message_handler::MarketDataHandler;

pub struct CoinbaseDataHandler {}

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
                println!("{}", msg_utf8);
                panic!("Could not deserialize message!");
            }
        };

        println!("Received message sequence {}", message.sequence_num);
        for event in message.events.iter() {
            match event {
                Event::MarketData(e) => {
                    match e {
                        update { product_id, updates } => {
                            println!("This update for {:?} contains {:?} update!", product_id, updates.iter().size_hint());
                        }
                        snapshot { product_id, updates } => {
                            println!("This update for {:?} contains {:?} update!", product_id, updates.iter().size_hint());
                        }
                    }
                }
                Event::Subscription(_) => {
                    println!("This is a subscription!");
                }
            }
        }
    }
}

impl CoinbaseDataHandler {
    fn handle_snapshot() {}

    fn handle_update() {}
}