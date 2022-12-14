use crate::{config::DbPool, plugins::ConnectorPlugins, repository::workspace_repo::Workspaces};
use actix_web::{
    web::{self, get, post, Data, Json, Path},
    HttpResponse, Responder, Scope,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

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
    _plugins: Data<ConnectorPlugins>,
    body: Json<ConnectionForm>,
    path: Path<Uuid>,
) -> impl Responder {
    let _form = body.into_inner();
    let ws_id = path.into_inner();
    let mut conn = pool.get().expect("error");
    let _ws = Workspaces::find(&mut conn, &ws_id).expect("msg");

    //let mut conn = pool.get().unwrap();

    HttpResponse::Ok().finish()
}

pub async fn all(
    _pool: Data<DbPool>,
    _plugins: Data<ConnectorPlugins>,
    _path: Path<Uuid>,
) -> impl Responder {
    HttpResponse::Ok().finish()
}
