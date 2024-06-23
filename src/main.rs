use actix::Actor;
use actix_http::encoding::Decoder;
use actix_web::{App, HttpServer, web};
use actix_web::dev::Payload;
use awc::ClientResponse;
use futures_util::{SinkExt as _, StreamExt as _};

use coinbase_connector::coinbase::coinbase_api::{connect_websocket, get_subscribe_message};
use coinbase_connector::coinbase::data_handler::CoinbaseDataHandler;
use coinbase_connector::coinbase::jwt::token::create_api_key;
use coinbase_connector::controller::order_book_controller::order_book_route;
use coinbase_connector::controller::status_controller::status;
use coinbase_connector::controller::websocket_controller::index;
use coinbase_connector::marketdata::order_book::Book;
use coinbase_connector::websocket::connection::{WebsocketClient, WebsocketMessage};
use coinbase_connector::websocket::message_handler::{MarketDataHandler, WebsocketMessageHandler};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let key = create_api_key();

    let message = get_subscribe_message(&key, vec!["BTC-USD".to_string()], "level2".to_string());
    let mut connection = connect_websocket().await;
    let (sink, stream) = connection.split();

    let boxed_handler: Box<dyn MarketDataHandler> = Box::new(CoinbaseDataHandler { order_book: Book::new("product".to_string())});

    let handler_addr = WebsocketMessageHandler { market_data_handler: boxed_handler }.start();

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
