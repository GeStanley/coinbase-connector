use bytes::Bytes;
use crate::websocket::message_handler::MarketDataHandler;

pub struct CoinbaseDataHandler {

}

impl MarketDataHandler for CoinbaseDataHandler {
    fn process_text(&mut self, bytes: Bytes) {
        println!("Received coinbase message!");
    }
}