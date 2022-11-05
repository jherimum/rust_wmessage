use diesel::{r2d2::ConnectionManager, PgConnection};
use dotenv::dotenv;
use log::info;
use r2d2::Pool;
use serde::Deserialize;

use anyhow::{Context, Result};

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub host: String,
    pub port: u16,
    pub database_url: String,
}

pub type DbPool = Pool<ConnectionManager<PgConnection>>;

impl AppConfig {
    pub fn from_env() -> Result<AppConfig> {
        dotenv().ok();

        info!("Loading configuration");
        config::Config::builder()
            .add_source(config::Environment::default())
            .build()
            .context("error while building config")?
            .try_deserialize::<AppConfig>()
            .context("error while deserializeing AppConfg")
    }

    pub async fn create_pool(&self) -> Result<DbPool> {
        info!("Creating database pool");
        let manager = ConnectionManager::<PgConnection>::new(self.database_url.to_string());
        Pool::builder()
            .build(manager)
            .context("error while building connection pool")
    }
}
