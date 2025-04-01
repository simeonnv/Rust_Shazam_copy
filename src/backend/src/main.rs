use actix_cors::Cors;
use actix_web::{App, HttpServer, http, middleware::Logger};
use env_logger::Env;
use sqlx::{Pool, Sqlite};
use tokio::sync::OnceCell;
use utils::db::init::{init_pool::init_pool, init_tables::init_tables};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

pub mod api_docs;
pub mod error;
pub mod routes;
pub mod utils;

static DB: OnceCell<Pool<Sqlite>> = OnceCell::const_new();

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    init_pool().await.expect("failed initing database and pool");
    init_tables().await.expect("failed to init tables");

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
            .service(routes::songs::songs())
            .service(
                SwaggerUi::new("/{_:.*}")
                    .url("/api-docs/openapi.json", api_docs::ApiDoc::openapi()),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
