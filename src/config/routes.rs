use actix_web::{HttpResponse, web};

use crate::api::*;

pub fn config_services(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("")
            .service(
                web::resource("/status")
                    .route(web::get().to(|| async { HttpResponse::Ok().body("alive") })),
            )
            .service(
                web::scope("/auth")
                    .service(
                        web::resource("/signup").route(web::post().to(user_controller::signup)),
                    )
                    .service(
                        web::resource("/login").route(web::post().to(user_controller::login)),
                    )
                    .service(
                        web::resource("/logout").route(web::post().to(user_controller::logout)),
                    )
            )
            .service(
                web::scope("/activity")
                    .service(
                        web::resource("")
                            .route(web::get().to(activity_controller::find_all))
                            .route(web::post().to(activity_controller::insert)),
                    )
                .service(
                    web::resource("/{id}")
                        .route(web::get().to(activity_controller::find_by_id))
                        .route(web::put().to(activity_controller::update))
                        .route(web::delete().to(activity_controller::delete))
                )
            )
        .service(
            web::scope("/event")
                .service(
                    web::resource("")
                        .route(web::get().to(event_controller::find_all))
                        .route(web::post().to(event_controller::insert)),
                )
                .service(
                    web::resource("/{id}")
                        .route(web::get().to(event_controller::find_by_id))
                        .route(web::put().to(event_controller::update))
                        .route(web::delete().to(event_controller::delete))
                )
        )
    );
}