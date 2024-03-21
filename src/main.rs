use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use controller::websocket_controller::{index};
use controller::status_controller::{hello, echo, manual_hello};

pub mod controller;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
            .route("/ws/", web::get().to(index))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
