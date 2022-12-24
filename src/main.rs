use actix_web::middleware::Logger;
use actix_web::web::{self, scope, Data};
use actix_web::{App, HttpResponse, HttpServer};

use log::info;

use wmessage::commons::config::AppConfig;
use wmessage::commons::error::IntoAppError;
use wmessage::commons::types::Result;
use wmessage::plugins::{smtp, ConnectorPlugins};
use wmessage::resources::registrations::{self};
use wmessage::resources::{apikeys, channels, connections, healths, messages, plugins};

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
                    .service(messages::routes())
                    .service(apikeys::create)
                    .service(healths::routes())
                    .service(plugins::routes())
                    .service(registrations::routes())
                    .service(channels::routes())
                    .service(web::resource("").route(web::get().to(index)))
                    .service(scope("/workspaces/{ws_id}").service(connections::routes())),
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
