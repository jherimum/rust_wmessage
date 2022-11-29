use actix_web::{get, web::Data, HttpResponse, Responder};
use serde::Serialize;

use crate::{
    app::State,
    plugins::{ConnectorPlugin, DispatchType, Property},
};

#[derive(Serialize)]
struct Plugin {
    name: String,
    properties: Vec<Property>,
    dispatchers: Vec<Dispatcher>,
}

#[derive(Serialize)]
struct Dispatcher {
    r#type: DispatchType,
    properties: Vec<Property>,
}

fn dispatchers(p: &dyn ConnectorPlugin) -> Vec<Dispatcher> {
    p.dispatchers()
        .values()
        .map(|d| Dispatcher {
            r#type: d.r#type(),
            properties: d.properties(),
        })
        .collect()
}

#[get("/api/plugins")]
pub async fn get(app_state: Data<State>) -> impl Responder {
    let plugins = &app_state.plugins;

    let mut response = Vec::new();

    for p in plugins.all() {
        response.push(Plugin {
            name: p.name(),
            properties: p.properties(),
            dispatchers: dispatchers(p),
        })
    }

    HttpResponse::Ok().json(response)
}
