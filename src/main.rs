use actix_web::middleware::Logger;
use actix_web::web::{self, scope, Data};
use actix_web::{App, HttpResponse, HttpServer};

use anyhow::{Context, Result};
use wmessage::app::routes::registrations::{self};
use wmessage::app::routes::{connections, plugins};
use wmessage::config::AppConfig;
use wmessage::plugins::{smtp, ConnectorPlugins};

#[actix_web::main]
async fn main() -> Result<()> {
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("debug"));

    let config =
        AppConfig::from_env().context("error while creating app config from environmeet")?;

    let pool = config
        .create_pool()
        .await
        .context("error while creating pool ")?;

    let smtp_plugin = smtp::StmpPlugin::new();

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
                    .service(plugins::routes())
                    .service(registrations::routes())
                    .service(web::resource("").route(web::get().to(index)))
                    .service(scope("/workspaces/{ws_id}").service(connections::routes())),
            )
    })
    .bind((config.host, config.port))?
    .run()
    .await?;

    Ok(())
}

async fn index() -> HttpResponse {
    HttpResponse::Ok().finish()
}
