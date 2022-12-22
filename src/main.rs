use actix_web::middleware::Logger;
use actix_web::web::{self, scope, Data};
use actix_web::{App, HttpResponse, HttpServer};

use log::info;
use wmessage::app::routes::health_routes::{self};
use wmessage::app::routes::registration_routes::{self};
use wmessage::app::routes::{apikey_routes, channel_routes, message_routes};
use wmessage::app::routes::{connection_routes, plugin_routes};
use wmessage::commons::config::AppConfig;
use wmessage::commons::error::IntoAppError;
use wmessage::commons::types::Result;
use wmessage::plugins::{smtp, ConnectorPlugins};

#[actix_web::main]
async fn main() -> Result<()> {
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
                    .service(message_routes::routes())
                    .service(apikey_routes::create)
                    .service(health_routes::routes())
                    .service(plugin_routes::routes())
                    .service(registration_routes::routes())
                    .service(channel_routes::routes())
                    .service(web::resource("").route(web::get().to(index)))
                    .service(scope("/workspaces/{ws_id}").service(connection_routes::routes())),
            )
    })
    .bind((config.host, config.port))
    .into_app_error()?
    .run()
    .await?;

    Ok(())
}

async fn index() -> HttpResponse {
    HttpResponse::Ok().finish()
}
