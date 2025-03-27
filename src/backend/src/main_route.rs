use actix_web::{HttpResponse, Responder, post, web};

#[post("/")]
pub async fn main_route(blob: web::Bytes) -> impl Responder {
    HttpResponse::Ok().content_type("audio/wav").body(blob)
}
