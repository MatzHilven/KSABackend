use actix_web::{web, HttpResponse, Result};

use crate::{
    config::db::Pool,
    constants,
    models::event::{NewEvent},
    models::response::ResponseBody,
    services::event_service
};

// GET api/event
pub async fn find_all(pool: web::Data<Pool>) -> Result<HttpResponse> {
    match event_service::find_all(&pool) {
        Ok(activities) => Ok(HttpResponse::Ok()
            .json(ResponseBody::new(constants::MESSAGE_OK, activities))),
        Err(err) => Ok(err.response())
    }
}

// GET api/event/{id}
pub async fn find_by_id(id: web::Path<i32>, pool: web::Data<Pool>) -> Result<HttpResponse> {
    match event_service::find_by_id(id.into_inner(), &pool) {
        Ok(activity) => Ok(HttpResponse::Ok().json(ResponseBody::new(constants::MESSAGE_OK, activity))),
        Err(err) => Ok(err.response()),
    }
}

// POST api/event
pub async fn insert(activity: web::Json<NewEvent>, pool: web::Data<Pool>) -> Result<HttpResponse> {
    match event_service::insert(activity.0, &pool) {
        Ok(()) => Ok(HttpResponse::Created()
            .json(ResponseBody::new(constants::MESSAGE_OK, constants::EMPTY))),
        Err(err) => Ok(err.response()),
    }
}

// PUT api/event/{id}
pub async fn update(id: web::Path<i32>, activity: web::Json<NewEvent>, pool: web::Data<Pool>) -> Result<HttpResponse> {
    match event_service::update(id.into_inner(), activity.0, &pool) {
        Ok(()) => {
            Ok(HttpResponse::Ok().json(ResponseBody::new(constants::MESSAGE_OK, constants::EMPTY)))
        }
        Err(err) => Ok(err.response()),
    }
}

// DELETE api/event/{id}
pub async fn delete(id: web::Path<i32>, pool: web::Data<Pool>) -> Result<HttpResponse> {
    match event_service::delete(id.into_inner(), &pool) {
        Ok(()) => {
            Ok(HttpResponse::Ok().json(ResponseBody::new(constants::MESSAGE_OK, constants::EMPTY)))
        }
        Err(err) => Ok(err.response()),
    }
}