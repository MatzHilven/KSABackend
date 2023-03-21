use actix_web::{
    get,
    post,
    put,
    delete,
    HttpResponse,
    web::Json,
    web::Path,
};
use diesel::prelude::*;

use crate::db::db::establish_connection;
use crate::models::event::{Event, EventInput, NewEvent};

#[get("/event")]
pub async fn get_events() -> HttpResponse {
    use crate::schema::events::dsl::*;

    let mut connection = establish_connection();

    let results = events
        .load::<Event>(&mut connection)
        .expect("Error loading events");

    HttpResponse::Ok().json(results)
}

#[get("/event/{id}")]
pub async fn get_event(path_id: Path<i32>) -> HttpResponse {
    use crate::schema::events::dsl::*;

    let mut connection = establish_connection();

    match events
        .filter(id.eq(path_id.into_inner()))
        .load::<Event>(&mut connection) {
        Ok(res) => HttpResponse::Ok().json(res),
        Err(_) => HttpResponse::NotFound().json("Event not found."),
    }
}

#[put("/event/{id}")]
pub async fn edit_event(path_id: Path<i32>, event: Json<EventInput>) -> HttpResponse {
    use crate::schema::events::dsl::*;

    let mut connection = establish_connection();

    let new_event = NewEvent {
        name: event.name.as_str(),
        image_url: event.image_url.as_str(),
        location: event.location.as_str(),
        start_date: event.start_date,
        end_date: event.end_date,
        link: event.link.as_deref(),
    };

    match diesel::update(events.filter(id.eq(path_id.into_inner())))
        .set(&new_event)
        .execute(&mut connection) {
        Ok(_) => HttpResponse::Ok().json("Successfully updated event."),
        Err(err) => HttpResponse::InternalServerError().json( err.to_string()),
    }
}

#[delete("/event/{id}")]
pub async fn delete_event(path_id: Path<i32>) -> HttpResponse {
    use crate::schema::events::dsl::*;

    let mut connection = establish_connection();

    match diesel::delete(events.filter(id.eq(path_id.into_inner())))
        .execute(&mut connection) {
        Ok(_) => HttpResponse::Ok().json("Successfully deleted event."),
        Err(err) => HttpResponse::InternalServerError().json( err.to_string()),
    }
}

#[post("/event")]
pub async fn add_event(event: Json<EventInput>) -> HttpResponse {
    use crate::schema::events::dsl::*;

    let mut connection = establish_connection();

    let new_event = NewEvent {
        name: event.name.as_str(),
        image_url: event.image_url.as_str(),
        location: event.location.as_str(),
        start_date: event.start_date,
        end_date: event.end_date,
        link: event.link.as_deref(),
    };

    match diesel::insert_into(events)
        .values(&new_event)
        .execute(&mut connection) {
        Ok(_) => HttpResponse::Created().json("Successfully added event."),
        Err(err) => HttpResponse::InternalServerError().json( err.to_string()),
    }
}
