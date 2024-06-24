use actix::Addr;
use actix_web::{get, HttpResponse, web, web::Path};
use log::info;

use crate::websocket::handlers::get_order_book::GetOrderBookRequest;
use crate::websocket::handlers::get_top_of_the_book::GetTopOfTheBookRequest;
use crate::websocket::message_handler::WebsocketMessageHandler;

#[get("/order-book/{product}")]
async fn order_book_route(svc: web::Data<Addr<WebsocketMessageHandler>>, path: Path<String>,) -> HttpResponse {
    let product = path.into_inner();

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

#[get("/top-of-the-book/{product}")]
async fn top_of_the_book_route(svc: web::Data<Addr<WebsocketMessageHandler>>, path: Path<String>,) -> HttpResponse {
    let product = path.into_inner();

    match svc.send(GetTopOfTheBookRequest {}).await {
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