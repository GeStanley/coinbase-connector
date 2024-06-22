use std::any::Any;
use std::error::Error;
use std::io::Read;

use actix::Actor;
use actix_http::encoding::Decoder;
use actix_web::{App, HttpServer, web};
use actix_web::dev::Payload;
use awc::ClientResponse;
use futures_util::{SinkExt as _, StreamExt as _};
use rand::RngCore;
use serde::{Deserialize, Serialize};

use controller::order_book_controller::order_book_route;
use controller::status_controller::status;
use controller::websocket_controller::index;

use crate::coinbase::coinbase_api::{connect_websocket, get_subscribe_message};
use crate::coinbase::handler::CoinbaseMarketDataHandler;
use crate::coinbase::jwt::token::create_api_key;
use crate::websocket::connection::WebsocketClient;
use crate::websocket::connection::WebsocketMessage;
use crate::websocket::market_data_handler::MarketDataHandler;

pub mod controller;
pub mod coinbase;
mod websocket;
mod marketdata;

async fn _handle_response(mut response: ClientResponse<Decoder<Payload>>) {
    let fn_name = "handle_response";
    println!("{}: Response: {:?}", fn_name, &response);


    match response.body().await {
        Ok(body) => {
            let _foo = body.to_vec();
            println!("==== BODY ====");
            println!("{:?}", String::from_utf8(_foo));
        },
        Err(_err) => {
            println!("error {:?}", _err);
        }
    }

}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let key = create_api_key();

    let message = get_subscribe_message(&key, vec!["BTC-USD".to_string()], "level2".to_string());
    let mut connection = connect_websocket().await;
    let (sink, stream) = connection.split();



    let handler_addr = MarketDataHandler {}.start();

    let addr = WebsocketClient::start(handler_addr, sink, stream);

    let _res = addr.send(WebsocketMessage {
        body: message
    }).await.unwrap();

    HttpServer::new(move || {
        App::new()
            .service(status)
            .service(order_book_route)
            .route("/ws/", web::get().to(index))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
