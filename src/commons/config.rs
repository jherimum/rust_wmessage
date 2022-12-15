use crate::commons::Result;
use crate::commons::{database::DbPool, error::IntoAppError};
use diesel::{r2d2::ConnectionManager, PgConnection};
use dotenv::dotenv;
use log::info;
use r2d2::Pool;
use serde::Deserialize;
use std::time::Duration;

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub host: String,
    pub port: u16,
    pub database_url: String,
}

impl AppConfig {
    pub fn from_env() -> Result<AppConfig> {
        dotenv().ok();

        info!("Loading configuration");
        config::Config::builder()
            .add_source(config::Environment::default())
            .build()
            .into_app_error()?
            .try_deserialize::<AppConfig>()
            .into_app_error()
    }

    pub async fn create_pool(&self) -> Result<DbPool> {
        info!("Creating database pool");
        let manager = ConnectionManager::<PgConnection>::new(self.database_url.to_string());
        Pool::builder()
            .connection_timeout(Duration::from_secs(10))
            .build(manager)
            .into_app_error()
    }
}
