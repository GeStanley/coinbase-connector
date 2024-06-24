use actix::prelude::*;
use bytes::Bytes;
use crate::marketdata::order_book::Book;

pub struct WebsocketMessageHandler {
    pub market_data_handler: Box<dyn MarketDataHandler>,
}

impl Actor for WebsocketMessageHandler {
    type Context = Context<Self>;
    fn started(&mut self, _ctx: &mut Self::Context) {
        println!("market data handler started!");
    }
}

pub trait MarketDataHandler: Unpin + 'static {

    fn process_text(&mut self, bytes: Bytes);

    fn get_order_book(&mut self) -> Book;
}

impl WebsocketMessageHandler {

    pub fn process_text_message(&mut self, bytes: Bytes) {
        self.market_data_handler.process_text(bytes);
    }

    pub fn get_order_book(&mut self) -> Book {
        self.market_data_handler.get_order_book()
    }
}
