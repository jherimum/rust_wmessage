use actix_web::middleware::Logger;
use actix_web::web::{self, scope, Data};
use actix_web::{App, HttpResponse, HttpServer};

use log::info;
use wmessage::app::routes::apikey;
use wmessage::app::routes::health::{self};
use wmessage::app::routes::registrations::{self};
use wmessage::app::routes::{connections, plugins};
use wmessage::commons::error::AppError;
use wmessage::config::AppConfig;
use wmessage::plugins::{smtp, ConnectorPlugins};

extern crate lazy_static;

#[actix_web::main]
async fn main() -> Result<(), AppError> {
    let config = AppConfig::from_env()?;
    let pool = config.create_pool().await?;
    let smtp_plugin = smtp::StmpPlugin::new();

    info!("Starting the server at {}:{}", config.host, config.port);

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            //.wrap(TracingLogger::default())
            .app_data(Data::new(pool.clone()))
            .app_data(Data::new(ConnectorPlugins::new(vec![Box::new(
                smtp_plugin.clone(),
            )])))
            .service(
                scope("/api")
                    .service(apikey::create)
                    .service(health::routes())
                    .service(plugins::routes())
                    .service(registrations::routes())
                    .service(web::resource("").route(web::get().to(index)))
                    .service(scope("/workspaces/{ws_id}").service(connections::routes())),
            )
    })
    .bind((config.host, config.port))
    .map_err(AppError::from)?
    .run()
    .await?;

    Ok(())
}

async fn index() -> HttpResponse {
    HttpResponse::Ok().finish()
}
