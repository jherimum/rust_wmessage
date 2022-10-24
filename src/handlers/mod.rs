pub mod registration;

use actix_web::{
    web::{self, ServiceConfig},
    HttpResponse,
};

use registration::register;

use crate::error::AppError;

#[warn(dead_code)]
type AppResult<T> = Result<T, AppError>;
#[warn(dead_code)]
type AppResponse = AppResult<HttpResponse>;

pub fn app_config(config: &mut ServiceConfig) {
    let registration = web::resource("/api/registrations").route(web::post().to(register));

    config.service(registration);
}
