use crate::{
    config::db::Pool,
    constants,
    models::{
        response::ResponseBody,
        user::{LoginDTO, UserDTO},
    },
    services::user_service,
};
use actix_web::{web, HttpRequest, HttpResponse, Result};

// POST api/auth/signup
pub async fn signup(user_dto: web::Json<UserDTO>, pool: web::Data<Pool>) -> Result<HttpResponse> {
    match user_service::signup(user_dto.0, &pool) {
        Ok(message) => Ok(HttpResponse::Ok().json(ResponseBody::new(&message, constants::EMPTY))),
        Err(err) => Ok(err.response()),
    }
}

// POST api/auth/login
pub async fn login(login_dto: web::Json<LoginDTO>, pool: web::Data<Pool>) -> Result<HttpResponse> {
    match user_service::login(login_dto.0, &pool) {
        Ok(token_res) => Ok(HttpResponse::Ok().json(ResponseBody::new(
            constants::MESSAGE_LOGIN_SUCCESS,
            token_res,
        ))),
        Err(err) => Ok(err.response()),
    }
}

// POST api/auth/logout
pub async fn logout(req: HttpRequest, pool: web::Data<Pool>) -> Result<HttpResponse> {
    if let Some(authen_header) = req.headers().get(constants::AUTHORIZATION) {
        match user_service::logout(authen_header, &pool) {
            Ok(_) => Ok(HttpResponse::Ok().json(ResponseBody::new(
                constants::MESSAGE_LOGOUT_SUCCESS,
                constants::EMPTY,
            ))),
            Err(err) => Ok(err.response()),
        }

    } else {
        Ok(HttpResponse::BadRequest().json(ResponseBody::new(
            constants::MESSAGE_TOKEN_MISSING,
            constants::EMPTY,
        )))
    }
}