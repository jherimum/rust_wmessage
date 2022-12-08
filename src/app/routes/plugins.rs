use std::ops::Deref;

use actix_web::{
    web::{self, get, resource, Data},
    HttpResponse, Scope,
};
use serde::Serialize;

use crate::plugins::{ConnectorPlugin, ConnectorPlugins, DispatchType, DispatcherPlugin, Property};

#[derive(Serialize)]
struct Plugin {
    name: String,
    properties: Vec<Property>,
    dispatchers: Vec<Dispatcher>,
}

impl Plugin {
    fn new(p: &dyn ConnectorPlugin) -> Self {
        Plugin {
            name: p.name(),
            properties: p.properties(),
            dispatchers: Plugin::dispatchers(p.deref()),
        }
    }

    fn dispatchers(p: &dyn ConnectorPlugin) -> Vec<Dispatcher> {
        p.dispatchers()
            .values()
            .map(|d| Dispatcher::new(*d))
            .collect()
    }
}

#[derive(Serialize)]
struct Dispatcher {
    r#type: DispatchType,
    properties: Vec<Property>,
}

impl Dispatcher {
    fn new(d: &dyn DispatcherPlugin) -> Self {
        Dispatcher {
            r#type: d.r#type(),
            properties: d.properties(),
        }
    }
}

pub fn routes() -> Scope {
    Scope::new("/plugins")
        .service(resource("").route(get().to(all)))
        .service(resource("/{name}").route(get().to(find_one)))
}

async fn all(plugins: Data<ConnectorPlugins>) -> HttpResponse {
    let body = plugins
        .all()
        .iter()
        .map(|p| Plugin::new(*p))
        .collect::<Vec<Plugin>>();

    HttpResponse::Ok().json(body)
}

async fn find_one(plugins: Data<ConnectorPlugins>, name: web::Path<String>) -> HttpResponse {
    let pl = plugins.get(name.into_inner()).map(|p| Plugin::new(p));
    match pl {
        Some(pl) => HttpResponse::Ok().json(pl),
        _ => HttpResponse::NotFound().finish(),
    }
}
