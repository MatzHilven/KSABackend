use actix_service::{forward_ready, Service, Transform};
use actix_web::{
    dev::{ServiceRequest, ServiceResponse},
    Error,
    http::{header, Method},
    HttpResponse, web::Data,
};
use actix_web::body::EitherBody;
use futures::{
    future::{ready, Ready},
    future::LocalBoxFuture,
};

use crate::{config::db::Pool, constants, models::response::ResponseBody, utils::token_utils};
use crate::models::user::User;

pub struct Authentication;

impl<S, B> Transform<S, ServiceRequest> for Authentication
    where
        S: Service<ServiceRequest, Response=ServiceResponse<B>, Error=Error>,
        S::Future: 'static,
        B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Transform = AuthenticationMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthenticationMiddleware { service }))
    }
}

pub struct AuthenticationMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for AuthenticationMiddleware<S>
    where
        S: Service<ServiceRequest, Response=ServiceResponse<B>, Error=Error>,
        S::Future: 'static,
        B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, mut req: ServiceRequest) -> Self::Future {
        let mut authenticate_pass: bool = false;
        let mut access_forbidden: bool = false;

        let headers = req.headers_mut();
        headers.append(
            header::CONTENT_LENGTH,
            header::HeaderValue::from_static("true"),
        );

        if Method::OPTIONS == *req.method() {
            authenticate_pass = true;
        } else {
            for ignore_route in constants::IGNORE_ROUTES.iter() {
                if req.method() == ignore_route.0 && req.path().starts_with(ignore_route.1) {
                    authenticate_pass = true;
                    break;
                }
            }

            if !authenticate_pass {
                if let Some(pool) = req.app_data::<Data<Pool>>() {
                    if let Some(authen_header) = req.headers().get(constants::AUTHORIZATION) {
                        if let Ok(authen_str) = authen_header.to_str() {
                            if authen_str.starts_with("Bearer") {
                                let token = authen_str[6..authen_str.len()].trim();
                                if let Ok(token_data) = token_utils::decode_token(token.to_string()) {
                                    if token_utils::verify_token(&token_data, pool).is_ok() {
                                        if let Ok(user) =
                                            User::find_user_by_username(
                                                token_data.claims.user.as_str(),
                                                &mut pool.get().unwrap()) {
                                            if user.can_access(&req) {
                                                authenticate_pass = true;
                                            } else {
                                                access_forbidden = true;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        if authenticate_pass {
            let fut = self.service.call(req);
            Box::pin(async move {
                fut.await.map(ServiceResponse::map_into_left_body)
            })
        } else if access_forbidden {
            Box::pin(async {
                Ok(ServiceResponse::new(req.into_parts().0, HttpResponse::Forbidden()
                    .json(ResponseBody::new(
                        constants::MESSAGE_FORBIDDEN,
                        constants::EMPTY,
                    ))).map_into_right_body())
            })
        } else {
            Box::pin(async {
                Ok(ServiceResponse::new(req.into_parts().0, HttpResponse::Unauthorized()
                    .json(ResponseBody::new(
                        constants::MESSAGE_INVALID_TOKEN,
                        constants::EMPTY,
                    ))).map_into_right_body())
            })
        }
    }
}