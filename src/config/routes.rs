use actix_web::{HttpResponse, web};

use crate::api::*;

pub fn config_services(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("")
            .service(
                web::resource("/")
                    .route(web::get().to(|| async { HttpResponse::Ok().body("alive") })),
            )
        // .service(
        //     web::scope("/auth")
        //         .service(
        //             web::resource("/signup").route(web::post().to(auth_controller::signup)),
        //         )
        //         .service(
        //             web::resource("/login").route(web::post().to(auth_controller::login)),
        //         )
        //         .service(
        //             web::resource("/logout").route(web::post().to(auth_controller::logout)),
        //         ),
        // .service(
        //     web::scope("/activity")
        //         .service(
        //             web::resource("")
        //                 .route(web::get().to(activity_controller::find_all))
        //                 .route(web::post().to(activity_controller::add_activity)),
        //         )
        // .service(
        //     web::resource("/{id}")
        //         .route(web::get().to(activity_controller::get_activity))
        //         .route(web::put().to(activity_controller::edit_activity))
        // )
        // )
        // .service(
        //     web::scope("/event")
        //         .service(
        //             web::resource("")
        //                 .route(web::get().to(event_controller::get_events))
        //                 .route(web::post().to(event_controller::add_event)),
        //         )
        //         .service(
        //             web::resource("/{id}")
        //                 .route(web::get().to(event_controller::get_event))
        //                 .route(web::put().to(event_controller::edit_event))
        //                 .route(web::delete().to(event_controller::delete_event))
        //         )
        // )
    );
}