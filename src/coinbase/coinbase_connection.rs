use actix::{Actor, Addr, Context};
use actix::io::SinkWrite;
use actix_codec::Framed;
use actix_http::ws::Codec;
use awc::BoxedSocket;
use futures_util::stream::{SplitSink, SplitStream};
use futures_util::StreamExt;
use log::info;

use crate::coinbase::coinbase_api::{connect_websocket, get_subscribe_message};
use crate::coinbase::data_handler::CoinbaseDataHandler;
use crate::coinbase::jwt::token::create_api_key;
use crate::marketdata::order_book::Book;
use crate::websocket::connection::{WebsocketClient, WebsocketMessage};
use crate::websocket::message_handler::{MarketDataHandler, WebsocketMessageHandler};

pub struct CoinbaseConnectionHandler {

}

impl CoinbaseConnectionHandler {
    pub async fn start(handler_addr: Addr<WebsocketMessageHandler>) -> Addr<Self> {
        info!("Starting coinbase connection...");
        let product = "BTC-USD".to_string();

        let key = create_api_key();
        let message = get_subscribe_message(&key, vec![product.clone()], "level2".to_string());

        let mut connection = connect_websocket().await;

        let (sink, stream) = connection.split();


        let addr = WebsocketClient::start(handler_addr, sink, stream);

        let _res = addr.do_send(WebsocketMessage {
            body: message
        });

        let connection = CoinbaseConnectionHandler {

        };
        info!("Coinbase connection handler started.");
        connection.start()
    }
}

impl Actor for CoinbaseConnectionHandler {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Context<Self>) {
        info!("Websocket client started");
    }
}