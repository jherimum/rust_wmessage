use super::rest::RestErrorKind;
use crate::commons::Result;
use crate::models::ModelErrorKind;
use actix_web::{error::UrlGenerationError, http::StatusCode, HttpResponse, ResponseError};
use config::ConfigError;
use serde::Serialize;
use std::fmt::Display;
use valico::json_schema::SchemaError;

pub trait IntoRestError<T> {
    fn into_not_found(self, message: &str) -> Result<T>;
}

impl<T, E: std::fmt::Debug + Into<AppError>> IntoRestError<T>
    for std::result::Result<Option<T>, E>
{
    fn into_not_found(self, message: &str) -> Result<T> {
        match self {
            Ok(Some(t)) => Ok(t),
            Ok(None) => Err(AppError {
                kind: crate::commons::error::AppErrorKind::RestError(
                    crate::commons::rest::RestErrorKind::NotFound,
                ),
                message: message.to_string(),
                cause: None,
            }),
            Err(e) => Err(e.into()),
        }
    }
}

pub trait IntoAppError<T> {
    fn into_app_error(self) -> Result<T>;
}

impl<T, E: std::fmt::Debug + Into<AppError>> IntoAppError<T> for Result<T, E> {
    fn into_app_error(self) -> Result<T> {
        match self {
            Ok(v) => Ok(v),
            Err(err) => Err(err.into()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AppErrorKind {
    IOError,
    PoolError,
    DatabaseError,
    EncryptionError,
    ModelError(ModelErrorKind),
    RestError(RestErrorKind),
    ConfigurationError,
    JsonSchemaError,
    JsonError,
    PluginError,
    UnexpectedError,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AppError {
    kind: AppErrorKind,
    message: String,
    cause: Option<String>,
}

#[derive(Debug, Serialize)]
struct AppErrorResponse {
    message: String,
}

impl From<AppError> for AppErrorResponse {
    fn from(err: AppError) -> Self {
        AppErrorResponse {
            message: err.message,
        }
    }
}

impl AppError {
    pub fn new(kind: AppErrorKind, mesage: &str, cause: Option<&str>) -> Self {
        AppError {
            kind: kind,
            message: mesage.to_string(),
            cause: cause.map(String::from),
        }
    }

    pub fn io_error(mesage: &str, cause: Option<&str>) -> Self {
        Self::new(AppErrorKind::IOError, mesage, cause)
    }

    pub fn pool_error(mesage: &str, cause: Option<&str>) -> Self {
        Self::new(AppErrorKind::PoolError, mesage, cause)
    }

    pub fn db_error(mesage: &str, cause: Option<&str>) -> Self {
        Self::new(AppErrorKind::DatabaseError, mesage, cause)
    }

    pub fn model_error(kind: ModelErrorKind) -> Self {
        Self::new(
            AppErrorKind::ModelError(kind.clone()),
            &kind.clone().to_string(),
            None,
        )
    }
    pub fn not_found(message: &str) -> AppError {
        AppError {
            kind: AppErrorKind::RestError(RestErrorKind::NotFound),
            message: message.to_string(),
            cause: None,
        }
    }

    pub fn database_error(cause: &str) -> Self {
        AppError {
            kind: AppErrorKind::DatabaseError,
            message: "Database error".to_string(),
            cause: Some(cause.to_string()),
        }
    }

    pub fn cause(self, cause: &str) -> Self {
        AppError {
            kind: self.kind,
            message: self.message,
            cause: Some(cause.to_string()),
        }
    }
}

impl Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl ResponseError for AppError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match &self.kind {
            AppErrorKind::ModelError(_) => StatusCode::CONFLICT,
            AppErrorKind::RestError(RestErrorKind::NotFound) => StatusCode::NOT_FOUND,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        HttpResponse::build(self.status_code()).json(AppErrorResponse::from(self.clone()))
    }
}

impl From<ConfigError> for AppError {
    fn from(err: ConfigError) -> Self {
        AppError {
            kind: AppErrorKind::ConfigurationError,
            message: "Configuration Error".to_string(),
            cause: Some(err.to_string()),
        }
    }
}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::io_error("Io error", Some(&err.to_string()))
    }
}

impl From<r2d2::Error> for AppError {
    fn from(err: r2d2::Error) -> Self {
        AppError {
            kind: AppErrorKind::PoolError,
            message: "Connection pool error".to_string(),
            cause: Some(err.to_string()),
        }
    }
}

impl From<serde_json::Error> for AppError {
    fn from(err: serde_json::Error) -> Self {
        AppError {
            kind: AppErrorKind::JsonError,
            message: "json error".to_string(),
            cause: Some(err.to_string()),
        }
    }
}

impl From<lettre::transport::smtp::Error> for AppError {
    fn from(err: lettre::transport::smtp::Error) -> Self {
        AppError {
            kind: AppErrorKind::PluginError,
            message: "Smtp error".to_string(),
            cause: Some(err.to_string()),
        }
    }
}

impl From<SchemaError> for AppError {
    fn from(err: SchemaError) -> Self {
        AppError {
            kind: AppErrorKind::JsonSchemaError,
            message: "Invalid schema".to_string(),
            cause: Some(err.to_string()),
        }
    }
}

impl From<diesel::result::Error> for AppError {
    fn from(err: diesel::result::Error) -> Self {
        AppError {
            kind: AppErrorKind::DatabaseError,
            message: "Database error".to_string(),
            cause: Some(err.to_string()),
        }
    }
}

impl From<argon2::password_hash::Error> for AppError {
    fn from(err: argon2::password_hash::Error) -> Self {
        AppError {
            kind: AppErrorKind::EncryptionError,
            message: "Encryption Error".to_string(),
            cause: Some(err.to_string()),
        }
    }
}

impl From<UrlGenerationError> for AppError {
    fn from(err: UrlGenerationError) -> Self {
        AppError {
            kind: AppErrorKind::UnexpectedError,
            message: "Not expected error".to_string(),
            cause: Some(err.to_string()),
        }
    }
}
