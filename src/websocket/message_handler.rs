use actix::prelude::*;
use bytes::Bytes;
use log::info;
use crate::coinbase::coinbase_connection::CoinbaseConnectionHandler;
use crate::coinbase::data_handler::CoinbaseDataHandler;

use crate::marketdata::order_book::Book;

pub struct WebsocketMessageHandler {
    market_data_handler: Box<dyn MarketDataHandler>,
}

impl WebsocketMessageHandler {
    pub fn start(product: String) -> Addr<Self> {

        let boxed_handler: Box<dyn MarketDataHandler> = Box::new(CoinbaseDataHandler { order_book: Book::new(product)});

        let handler = WebsocketMessageHandler {
            market_data_handler: boxed_handler,
        };
        info!("Websocket message handler started.");
        handler.start()
    }
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

    fn get_top_of_the_book(&mut self) -> Book;
}

impl WebsocketMessageHandler {

    pub fn process_text_message(&mut self, bytes: Bytes) {
        self.market_data_handler.process_text(bytes);
    }

    pub fn get_order_book(&mut self) -> Book {
        self.market_data_handler.get_order_book()
    }

    pub fn get_top_of_the_book(&mut self) -> Book {
        self.market_data_handler.get_top_of_the_book()
    }
}
