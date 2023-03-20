extern crate diesel;
extern crate dotenv;

use actix_cors::Cors;
use actix_web::{App, HttpServer, middleware::Logger};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

use crate::api::api::{alive, add_activity, get_activities};

mod db;
mod schema;
mod api;
mod models;
mod repository;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file("key.pem", SslFiletype::PEM)
        .unwrap();
    builder.set_certificate_chain_file("cert.pem").unwrap();

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
            .service(alive)
            .service(add_activity)
            .service(get_activities)
    })
        .bind_openssl("127.0.0.1:443", builder)?
        .run()
        .await
}