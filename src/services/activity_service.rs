use crate::{
    config::db::Pool,
    constants,
    error::ServiceError,
    models::activity::{Activity},
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