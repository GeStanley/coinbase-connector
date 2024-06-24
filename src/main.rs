use actix::Actor;
use actix_web::{App, HttpServer};

use coinbase_connector::coinbase::coinbase_connection::CoinbaseConnectionHandler;
use coinbase_connector::controller::order_book_controller::order_book_route;
use coinbase_connector::controller::status_controller::status;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    CoinbaseConnectionHandler::start().await;

    HttpServer::new(move || {
        App::new()
            .service(status)
            .service(order_book_route)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
