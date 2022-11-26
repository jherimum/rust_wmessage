use std::collections::HashMap;

use actix_web::get;
use actix_web::middleware::Logger;
use actix_web::web::Data;
use actix_web::{App, HttpServer};

use anyhow::{Context, Result};
use wmessage::app::routes::registrations::register;
use wmessage::app::State;
use wmessage::config::AppConfig;
use wmessage::plugins::smtp;
use wmessage::plugins::ConnectorPlugins;

#[actix_web::main]
async fn main() -> Result<()> {
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("debug"));
    let config =
        AppConfig::from_env().context("error while creating app config from environmeet")?;
    let pool = config
        .create_pool()
        .await
        .context("error while creating pool ")?;

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            //.wrap(TracingLogger::default())
            .app_data(Data::new(State {
                pool: pool.clone(),
                plugins: ConnectorPlugins::new(vec![Box::new(smtp::Plugin::new())]),
            }))
            .service(register)
        //.service(register)
    })
    .bind((config.host, config.port))?
    .run()
    .await?;

    Ok(())
}

#[get("/")]
async fn index() -> String {
    format!("{}", "index")
}
