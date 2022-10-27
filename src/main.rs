use actix_web::web::Data;
use actix_web::{middleware::Logger, App, HttpServer};
use wmessage::app::State;

use wmessage::app::handlers::registrations::register;
use wmessage::config::AppConfig;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("debug"));

    let config = AppConfig::from_env().expect("Server configuration");
    let pool = config.create_pool().await.expect("Database");

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(Data::new(State { pool: pool.clone() }))
            .service(register)
    })
    .bind((config.host, config.port))?
    .run()
    .await
}
