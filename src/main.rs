extern crate diesel;
extern crate dotenv;

use actix_cors::Cors;
use actix_web::{App, HttpResponse, HttpServer, middleware::Logger, http};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

use crate::api::activity::{add_activity, get_activities, get_activity};
use crate::api::event::{get_event, get_events, edit_event, add_event, delete_event};

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
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);

        App::new()
            .wrap(logger)
            .wrap(cors)
            .route("/", actix_web::web::to(|| async { HttpResponse::Ok().body("alive") }))
            .service(add_activity)
            .service(get_activities)
            .service(get_activity)

            .service(get_event)
            .service(get_events)
            .service(edit_event)
            .service(add_event)
            .service(delete_event)
    })
        .bind_openssl("127.0.0.1:443", builder)?
        .run()
        .await
}