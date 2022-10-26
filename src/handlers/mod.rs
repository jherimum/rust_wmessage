pub mod channels;
pub mod registrations;

use actix_web::{
    web::{self, ServiceConfig},
    HttpResponse,
};

use registrations::register;

use crate::error::AppError;

use self::channels::create_channel;

#[warn(dead_code)]
type AppResult<T> = Result<T, AppError>;
#[warn(dead_code)]
type AppResponse = AppResult<HttpResponse>;

pub fn app_config(config: &mut ServiceConfig) {
    let registration = web::resource("/api/registrations").route(web::post().to(register));
    let create_channel = web::resource("/api/workspaces/{workspace_id}/channels")
        .route(web::post().to(create_channel));
    config.service(registration).service(create_channel);
}
