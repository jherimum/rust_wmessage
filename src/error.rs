use std::fmt::Display;

use actix_web::error;

impl error::ResponseError for AppError {}

#[derive(Debug)]
pub enum AppError {
    DatabaseError(diesel::result::Error),
    PoolError(r2d2::Error),
    WorkspaceWithCodeAlreadyExistsError,
    Unknow,
}

impl std::error::Error for AppError {}

impl Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl From<diesel::result::Error> for AppError {
    fn from(e: diesel::result::Error) -> Self {
        AppError::DatabaseError(e)
    }
}