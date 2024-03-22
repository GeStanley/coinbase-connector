use actix::Actor;
use actix_web::{web, App, HttpServer};
use actix_web::web::Data;
use awc::{Client, ClientBuilder};
use coinbase::lobby::Lobby;
use coinbase::start_connection::start_connection as start_connection_route;
use controller::websocket_controller::{index};
use controller::status_controller::{hello, echo, manual_hello};
use futures_util::{SinkExt as _, StreamExt as _};
use openssl::ssl::{SslConnector, SslConnectorBuilder, SslMethod};

pub mod controller;
pub mod coinbase;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    // let builder = SslConnector::builder(SslMethod::tls()).unwrap();

    // ClientBuilder::new().connector(awc::Connector::new().ssl(builder.build()).finish();


    let (_resp, mut connection) = match Client::builder().max_http_version(awc::http::Version::HTTP_11).finish()
        .ws("wss://ws-feed-public.sandbox.exchange.coinbase.com")
        .connect()
        .await {
        Ok((_resp, connection)) => (_resp, connection),
        Err(error) => {
            println!("Error: {}", error);
            panic!("Problem creating websocket connection.");
        },
    };

    match connection
        .send(awc::ws::Message::Text("Echo".into()))
        .await {
        Ok(..) => println!("Message sent."),
        Err(error) => {
            println!("Error: {}", error);
            panic!("Problem sending on websocket connection.");
        },
    };

    let response = match connection.next().await.unwrap() {
        Ok(response) => response,
        Err(_error) => panic!("Did not get a response from websocket connection."),
    };
    println!("{:?}", response);
    assert_eq!(response, awc::ws::Frame::Text("Echo".into()));


    let chat_server = Lobby::default().start();

    HttpServer::new(move || {
        App::new()
            .service(hello)
            .service(echo)
            .service(start_connection_route)
            .app_data(Data::new(chat_server.clone()))
            .route("/hey", web::get().to(manual_hello))
            .route("/ws/", web::get().to(index))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
