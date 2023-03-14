use actix_cors::Cors;
use actix_web::{App, HttpServer, middleware::Logger, web::Data};

mod api;
mod models;
mod repository;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    HttpServer::new(move || {
        let logger = Logger::default();

        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_header()
            .allowed_methods(vec!["GET", "POST"])
            .supports_credentials()
            .max_age(3600);

        App::new()
            .wrap(logger)
            .wrap(cors)
    })
        .bind(("0.0.0.0", 80))?
        .run()
        .await
}