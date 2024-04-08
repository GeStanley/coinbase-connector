use actix_web::{get, web::Path, HttpResponse, Responder};

#[get("/{product}")]
async fn order_book_route(    path: Path<String>,) -> impl Responder {
    let product = path.into_inner();
    HttpResponse::Ok().body(product)
}