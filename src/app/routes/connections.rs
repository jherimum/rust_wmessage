use std::collections::HashMap;

use actix_web::{
    post,
    web::{self, get, post, resource, Data, Json, Path},
    HttpResponse, Responder, Scope,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{config::DbPool, models::workspace::Workspace, plugins::ConnectorPlugins};

#[derive(Serialize, Deserialize)]
pub struct ConnectionForm {
    code: String,
    name: String,
    plugin_name: String,
    properties: HashMap<String, serde_json::Value>,
}

pub fn routes() -> Scope {
    Scope::new("/connections").service(
        web::resource("")
            .route(post().to(create))
            .route(get().to(all)),
    )
}

pub async fn create(
    pool: Data<DbPool>,
    plugins: Data<ConnectorPlugins>,
    body: Json<ConnectionForm>,
    path: Path<Uuid>,
) -> impl Responder {
    let form = body.into_inner();
    let ws_id = path.into_inner();
    let mut conn = pool.get().expect("error");
    let ws = Workspace::find(&mut conn, &ws_id).expect("msg");

    //let mut conn = pool.get().unwrap();

    HttpResponse::Ok().finish()
}

pub async fn all(
    pool: Data<DbPool>,
    plugins: Data<ConnectorPlugins>,
    path: Path<Uuid>,
) -> impl Responder {
    HttpResponse::Ok().finish()
}
