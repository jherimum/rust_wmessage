extern crate derive_more;

use actix_web::{web, HttpResponse};

use crate::services::registration::{RegistrationForm, RegistrationService};

pub async fn register(
    form: web::Json<RegistrationForm>,
    service: RegistrationService,
) -> HttpResponse {
    match service.register(form.into_inner()) {
        Ok(()) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::Conflict().finish(),
    }
}