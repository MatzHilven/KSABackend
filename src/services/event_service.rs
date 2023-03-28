use crate::{
    config::db::Pool,
    constants,
    error::ServiceError,
    models::event::{Event, NewEvent},
};
use actix_web::{http::StatusCode, web};

pub fn find_all(pool: &web::Data<Pool>) -> Result<Vec<Event>, ServiceError> {
    match Event::find_all(&mut pool.get().unwrap()) {
        Ok(event) => Ok(event),
        Err(_) => Err(ServiceError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            constants::MESSAGE_CAN_NOT_FETCH_DATA.to_string(),
        )),
    }
}

pub fn find_by_id(id: i32, pool: &web::Data<Pool>) -> Result<Event, ServiceError> {
    match Event::find_by_id(id, &mut pool.get().unwrap()) {
        Ok(event) => Ok(event),
        Err(_) => Err(ServiceError::new(
            StatusCode::NOT_FOUND,
            format!("Event with id {} not found", id),
        )),
    }
}

pub fn insert(new_event: NewEvent, pool: &web::Data<Pool>) -> Result<(), ServiceError> {
    match Event::insert(new_event, &mut pool.get().unwrap()) {
        Ok(_) => Ok(()),
        Err(_) => Err(ServiceError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            constants::MESSAGE_CAN_NOT_INSERT_DATA.to_string(),
        )),
    }
}

pub fn update(id: i32, updated_event: NewEvent, pool: &web::Data<Pool>) -> Result<(), ServiceError> {
    match Event::find_by_id(id, &mut pool.get().unwrap()) {
        Ok(_) => match Event::update(id, updated_event, &mut pool.get().unwrap()) {
            Ok(_) => Ok(()),
            Err(_) => Err(ServiceError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                constants::MESSAGE_CAN_NOT_UPDATE_DATA.to_string(),
            )),
        },
        Err(_) => Err(ServiceError::new(
            StatusCode::NOT_FOUND,
            format!("Event with id {} not found", id),
        )),
    }
}

pub fn delete(id: i32, pool: &web::Data<Pool>) -> Result<(), ServiceError> {
    match Event::find_by_id(id, &mut pool.get().unwrap()) {
        Ok(_) => match Event::delete(id, &mut pool.get().unwrap()) {
            Ok(_) => Ok(()),
            Err(_) => Err(ServiceError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                constants::MESSAGE_CAN_NOT_DELETE_DATA.to_string(),
            )),
        },
        Err(_) => Err(ServiceError::new(
            StatusCode::NOT_FOUND,
            format!("Event with id {} not found", id),
        )),
    }
}