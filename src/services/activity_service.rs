use crate::{
    config::db::Pool,
    constants,
    error::ServiceError,
    models::activity::{Activity, ActivityDTO},
};
use actix_web::{http::StatusCode, web};

pub fn find_all(pool: &web::Data<Pool>) -> Result<Vec<Activity>, ServiceError> {
    match Activity::find_all(&mut pool.get().unwrap()) {
        Ok(activity) => Ok(activity),
        Err(_) => Err(ServiceError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            constants::MESSAGE_CAN_NOT_FETCH_DATA.to_string(),
        )),
    }
}

pub fn find_by_id(id: i32, pool: &web::Data<Pool>) -> Result<Activity, ServiceError> {
    match Activity::find_by_id(id, &mut pool.get().unwrap()) {
        Ok(activity) => Ok(activity),
        Err(_) => Err(ServiceError::new(
            StatusCode::NOT_FOUND,
            format!("Activity with id {} not found", id),
        )),
    }
}

pub fn insert(new_activity: ActivityDTO, pool: &web::Data<Pool>) -> Result<(), ServiceError> {
    match Activity::insert(new_activity, &mut pool.get().unwrap()) {
        Ok(_) => Ok(()),
        Err(_) => Err(ServiceError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            constants::MESSAGE_CAN_NOT_INSERT_DATA.to_string(),
        )),
    }
}

pub fn update(id: i32, updated_activity: ActivityDTO, pool: &web::Data<Pool>) -> Result<(), ServiceError> {
    match Activity::find_by_id(id, &mut pool.get().unwrap()) {
        Ok(_) => match Activity::update(id, updated_activity, &mut pool.get().unwrap()) {
            Ok(_) => Ok(()),
            Err(_) => Err(ServiceError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                constants::MESSAGE_CAN_NOT_UPDATE_DATA.to_string(),
            )),
        },
        Err(_) => Err(ServiceError::new(
            StatusCode::NOT_FOUND,
            format!("Activity with id {} not found", id),
        )),
    }
}

pub fn delete(id: i32, pool: &web::Data<Pool>) -> Result<(), ServiceError> {
    match Activity::find_by_id(id, &mut pool.get().unwrap()) {
        Ok(_) => match Activity::delete(id, &mut pool.get().unwrap()) {
            Ok(_) => Ok(()),
            Err(_) => Err(ServiceError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                constants::MESSAGE_CAN_NOT_DELETE_DATA.to_string(),
            )),
        },
        Err(_) => Err(ServiceError::new(
            StatusCode::NOT_FOUND,
            format!("Activity with id {} not found", id),
        )),
    }
}