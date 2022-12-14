use super::rest::RestErrorKind;
use crate::models::ModelErrorKind;
use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use config::ConfigError;
use serde::Serialize;
use std::fmt::Display;
use valico::json_schema::SchemaError;

pub trait IntoAppError<T> {
    fn into_app_error(self) -> core::result::Result<T, AppError>;
}

impl<T, E: std::fmt::Debug + Into<AppError>> IntoAppError<T> for core::result::Result<T, E> {
    fn into_app_error(self) -> core::result::Result<T, AppError> {
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
    pub fn not_found(message: &str) -> AppError {
        AppError {
            kind: AppErrorKind::RestError(RestErrorKind::NotFound),
            message: message.to_string(),
            cause: None,
        }
    }

    pub fn model_error(kind: ModelErrorKind) -> Self {
        AppError {
            kind: AppErrorKind::ModelError(kind.clone()),
            message: kind.to_string(),
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
        AppError {
            kind: AppErrorKind::IOError,
            message: "Io error".to_string(),
            cause: Some(err.to_string()),
        }
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
