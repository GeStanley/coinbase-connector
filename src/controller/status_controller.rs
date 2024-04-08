use actix_web::{get, HttpResponse, Responder};

#[get("/status")]
async fn status() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}
