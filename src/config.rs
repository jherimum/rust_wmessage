use std::future::{ready, Ready};

use actix_web::FromRequest;
use diesel::{r2d2::ConnectionManager, PgConnection};
use dotenv::dotenv;
use log::info;
use r2d2::Pool;
use serde::Deserialize;

use crate::error::AppError;

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub host: String,
    pub port: u16,
    pub database_url: String,
}

pub type DbPool = Pool<ConnectionManager<PgConnection>>;

impl AppConfig {
    pub fn from_env() -> Result<AppConfig, AppError> {
        dotenv().ok();

        info!("Loading configuration");

        config::Config::builder()
            .add_source(config::Environment::default())
            .build()
            .unwrap()
            .try_deserialize::<AppConfig>()
            .map_err(|_e| AppError::Unknow)
    }

    pub async fn create_pool(&self) -> Result<DbPool, AppError> {
        info!("Creating database pool");
        let manager = ConnectionManager::<PgConnection>::new(self.database_url.to_string());
        Pool::builder()
            .build(manager)
            .map_err(|e| AppError::PoolError(e))
    }
}