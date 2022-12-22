pub mod config;
pub mod database;
pub mod encrypt;
pub mod error;
pub mod id;
pub mod json;
pub mod rest;
pub mod validators;

use self::error::AppError;
use chrono::NaiveDateTime;

pub type Result<T, E = AppError> = std::result::Result<T, E>;

pub type Timestamp = NaiveDateTime;

pub type Id = uuid::Uuid;
