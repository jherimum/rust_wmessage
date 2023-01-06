use actix_web::middleware::Logger;
use actix_web::web::{scope, Data};
use actix_web::{App, HttpServer};
use log::info;
use wmessage::commons::config::AppConfig;
use wmessage::commons::error::IntoAppError;
use wmessage::commons::types::Result;
use wmessage::plugins::{smtp, ConnectorPlugins};
use wmessage::resources::registrations::{self};
use wmessage::resources::{
    apikeys, channels, connections, healths, message_types, messages, plugins,
};

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
                    .service(messages::resources())
                    .service(apikeys::create)
                    .service(healths::routes())
                    .service(plugins::routes())
                    .service(registrations::routes())
                    .service(message_types::routes())
                    .service(channels::resources())
                    .service(scope("/workspaces/{ws_id}").service(connections::routes())),
            )
    })
    .bind((config.host, config.port))
    .into_app_error()?
    .run()
    .await?;

    Ok(())
}
