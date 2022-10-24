use actix_web::{middleware::Logger, web, App, HttpServer};
use wmessage::{config::AppConfig, handlers::app_config, services::crypto::PasswordCrypto};

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("debug"));

    let config = AppConfig::from_env().expect("Server configuration");
    let pool = config.create_pool().await.expect("Database");

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(PasswordCrypto::new()))
            .configure(app_config)
    })
    .bind((config.host, config.port))?
    .run()
    .await
}
