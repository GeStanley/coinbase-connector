use std::fs;
use actix::Addr;

use actix_web::{get, HttpResponse, web, web::Path};

use crate::coinbase::api::websocket::CoinbaseWebsocketMessage;
use crate::coinbase::data_handler::CoinbaseDataHandler;
use crate::marketdata::order_book::Book;
use crate::websocket::handlers::get_order_book::{GetOrderBookRequest, GetOrderBookResponse};
use crate::websocket::message_handler::WebsocketMessageHandler;

#[get("/order-book/{product}")]
async fn order_book_route(svc: web::Data<Addr<WebsocketMessageHandler>>, path: Path<String>,) -> HttpResponse {
    let product = path.into_inner();

    // let snapshot = "tests/resources/snapshot.json";
    // let snapshot_contents = fs::read_to_string(snapshot);
    // let websocket_snapshot: CoinbaseWebsocketMessage = serde_json::from_str(&*snapshot_contents.unwrap()).unwrap();
    //
    // let mut data_handler = CoinbaseDataHandler { order_book: Book::new(product.clone())};
    // data_handler.handle_coinbase_websocket_message(websocket_snapshot);

    match svc.send(GetOrderBookRequest {}).await {
        Ok(resp) => {
            match resp {
                Ok(book) => {
                    HttpResponse::Ok().json(web::Json(book))
                }
                Err(_) => {
                    HttpResponse::Ok().json("error")
                }
            }

        }
        Err(_) => {
            HttpResponse::Ok().json("error")
        }
    }


}