use actix_web::{App, HttpServer};
pub mod main_route;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(main_route::main_route)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}