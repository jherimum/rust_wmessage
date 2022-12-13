use std::ops::DerefMut;

use actix_web::HttpResponse;
use diesel::PgConnection;
use thiserror::Error;

use crate::{commons::error::AppError, models::workspace::Workspace};

extern crate derive_more;
use derive_more::{Display, From, Into};

pub mod apikey;
pub mod channels;
pub mod connections;
pub mod health;
pub mod plugins;
pub mod registrations;

pub fn find_workspace(conn: &mut PgConnection, id: uuid::Uuid) -> Result<Workspace, AppError> {
    match Workspace::find(conn, &id)? {
        Some(ws) => Ok(ws),
        None => Err(AppError::not_found("Workspace not found")),
    }
}

/*
#[derive(Debug, Display)]
struct MyError {
    err: anyhow::Error,
}

impl actix_web::error::ResponseError for MyError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        actix_web::http::StatusCode::INTERNAL_SERVER_ERROR
    }

    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        HttpResponse::InternalServerError().finish()
    }
}

impl From<anyhow::Error> for MyError {
    fn from(err: anyhow::Error) -> MyError {
        MyError { err }
    }
}
 */
