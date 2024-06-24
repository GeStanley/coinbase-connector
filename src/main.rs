use actix::Actor;
use actix_web::{App, HttpServer, web};
use coinbase_connector::coinbase::coinbase_api::send_http_request;

use coinbase_connector::coinbase::coinbase_connection::CoinbaseConnectionHandler;
use coinbase_connector::coinbase::jwt::token::create_api_key;
use coinbase_connector::controller::order_book_controller::{order_book_route, top_of_the_book_route};
use coinbase_connector::controller::status_controller::status;
use coinbase_connector::websocket::message_handler::WebsocketMessageHandler;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let message_handler = WebsocketMessageHandler::start("text product".to_string());

    CoinbaseConnectionHandler::start(message_handler.clone()).await;

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(message_handler.clone()))
            .service(status)
            .service(order_book_route)
            .service(top_of_the_book_route)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
