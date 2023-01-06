use super::error::AppError;
use chrono::NaiveDateTime;
use diesel::{r2d2::ConnectionManager, PgConnection};
use r2d2::Pool;

pub type Result<T, E = AppError> = std::result::Result<T, E>;

pub type Timestamp = NaiveDateTime;

pub type Id = uuid::Uuid;

pub type DbPool = Pool<ConnectionManager<PgConnection>>;

pub type Version = i32;

pub type Code = String;

pub type Conn = PgConnection;

pub type Json = serde_json::Value;
