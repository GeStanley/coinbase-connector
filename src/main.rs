use std::any::Any;
use std::error::Error;
use std::io::Read;

use actix::Actor;
use actix_http::encoding::Decoder;
use actix_web::{App, HttpServer, web, web::Data};
use actix_web::dev::Payload;
use awc::{Client, ClientResponse, ws};
use awc::ws::Message;
use bytestring::ByteString;
use futures_util::{SinkExt as _, StreamExt as _};
use rand::RngCore;
use serde::{Deserialize, Serialize};

use coinbase::lobby::Lobby;
use coinbase::start_connection::start_connection as start_connection_route;
use controller::order_book_controller::order_book_route;
use controller::status_controller::status;
use controller::websocket_controller::index;
use crate::coinbase::coinbase_api::connect_websocket;

use crate::coinbase::coinbase_ws::CoinbaseConn;
// use crate::coinbase::coinbase_ws::CoinbaseConn::subscribe;
use crate::coinbase::feed::CoinbaseMarketData;
use crate::coinbase::jwt::token::create_api_key;

pub mod controller;
pub mod coinbase;
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

    connect_websocket(&key).await;
    // echo_server().await;


    let (_res, mut connection) = match Client::builder().max_http_version(awc::http::Version::HTTP_11).finish()
        .ws("wss://advanced-trade-ws.coinbase.com")
        .max_frame_size(6000_000)
        .connect()
        .await {
        Ok((_resp, connection)) => (_resp, connection),
        Err(error) => {
            println!("Error: {}", error);
            panic!("Problem creating websocket connection.");
        },
    };

    let market_data_feed = CoinbaseMarketData::default().start();
    let coinbase_connection = CoinbaseConn::new(key, market_data_feed, connection).start();
    coinbase_connection.subscribe("BTC-USD", "level2");

    let chat_server = Lobby::default().start();

    HttpServer::new(move || {
        App::new()
            .service(start_connection_route)
            .service(status)
            .service(order_book_route)
            .app_data(Data::new(chat_server.clone()))
            .route("/ws/", web::get().to(index))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

pub async fn echo_server() {
    let (_res, mut connection) = match Client::builder().max_http_version(awc::http::Version::HTTP_11).finish()
        .ws("wss://echo.websocket.org/")
        .connect()
        .await {
        Ok((_resp, connection)) => (_resp, connection),
        Err(error) => {
            println!("Error: {}", error);
            panic!("Problem creating websocket connection.");
        },
    };

    println!("{:?}", _res);

    let res = match connection
        .send(Message::Text(ByteString::from("test")))
        .await {
        Ok(res) => {
            println!("Message sent.");
            res
        },
        Err(error) => {
            println!("Error: {}", error);
            panic!("Problem sending on websocket connection.");
        },
    };

    match connection
        .send(Message::Text(ByteString::from("test2")))
        .await {
        Ok(res) => {
            println!("Message sent.");
            res
        },
        Err(error) => {
            println!("Error: {}", error);
            panic!("Problem sending on websocket connection.");
        },
    };

    while let Some(Ok(frame)) = connection.next().await {
        match frame {
            ws::Frame::Text(text) => {
                println!("Received Text: {:?}", text);
            }
            _ => {
                println!("Received Something else...")
            },
        }
    }
}