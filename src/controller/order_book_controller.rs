use std::fs;

use actix_web::{get, HttpResponse, web, web::Path};

use crate::coinbase::api::websocket::CoinbaseWebsocketMessage;
use crate::coinbase::data_handler::CoinbaseDataHandler;
use crate::marketdata::order_book::Book;

#[get("/order-book/{product}")]
async fn order_book_route(path: Path<String>,) -> HttpResponse {
    let product = path.into_inner();

    let snapshot = "tests/resources/snapshot.json";
    let snapshot_contents = fs::read_to_string(snapshot);
    let websocket_snapshot: CoinbaseWebsocketMessage = serde_json::from_str(&*snapshot_contents.unwrap()).unwrap();

    let mut data_handler = CoinbaseDataHandler { order_book: Book::new(product.clone())};
    data_handler.handle_coinbase_websocket_message(websocket_snapshot);

    HttpResponse::Ok().json(web::Json(data_handler.order_book))
}