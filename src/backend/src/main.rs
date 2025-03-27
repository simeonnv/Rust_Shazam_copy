use actix_cors::Cors;
use actix_web::{App, HttpServer, http, middleware::Logger};
use env_logger::Env;
pub mod main_route;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(|| {
        App::new()
            .wrap(
                Cors::default()
                    .allowed_origin("http://localhost:5173") // Allow your frontend origin
                    .allowed_methods(vec!["POST"]) // Allow POST requests
                    .allowed_headers(vec![http::header::CONTENT_TYPE]) // Allow Content-Type header
                    .max_age(3600), // Cache preflight for 1 hour
            )
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .service(main_route::main_route)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
