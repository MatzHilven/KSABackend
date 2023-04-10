extern crate actix_web;
extern crate log;
extern crate diesel;
extern crate diesel_migrations;
extern crate serde_json;
extern crate actix_cors;
extern crate actix_rt;
extern crate bcrypt;
extern crate derive_more;
extern crate dotenv;
extern crate env_logger;
extern crate failure;
extern crate futures;
extern crate jsonwebtoken;
#[macro_use]
extern crate serde;
extern crate uuid;

use actix_service::Service;
use actix_cors::Cors;
use actix_web::{App, HttpServer, http};
use actix_web::web::Data;
use crate::futures::FutureExt;
use dotenv::dotenv;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

mod controllers;
mod config;
mod constants;
mod error;
mod middleware;
mod models;
mod schema;
mod services;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder.set_private_key_file("key.pem", SslFiletype::PEM).unwrap();
    builder.set_certificate_chain_file("cert.pem").unwrap();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = config::db::migrate_and_config_db(&database_url);

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://127.0.0.1:443")
            .allowed_origin("http://localhost:443")
            .send_wildcard()
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);

        App::new()
            .wrap(cors)
            .app_data(Data::new(pool.clone()))
            .wrap(actix_web::middleware::Logger::default())
            .wrap(middleware::auth::Authentication)
            .wrap_fn(|req, srv| srv.call(req).map(|res| res))
            .configure(config::routes::config_services)
    })
        .bind_openssl("127.0.0.1:443", builder)?
        .run()
        .await
}