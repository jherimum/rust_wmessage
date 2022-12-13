use actix_web::{
    web::{self, get, patch, post, Data, Json},
    HttpResponse, Scope,
};
use serde::{Deserialize, Serialize};

use super::find_workspace;
use crate::{commons::error::AppError, config::DbPool, models::channel::Channel};

pub fn routes() -> Scope {
    let channels = web::resource("")
        .route(post().to(create))
        .route(get().to(all));
    let channel = web::resource("/{channel_id}")
        .route(get().to(find))
        .route(patch().to(update));

    Scope::new("/workspaces/{ws_id}/channels")
        .service(channels)
        .service(channel)
}

#[derive(Deserialize, Debug, Clone)]
struct ChannelForm {
    code: String,
    description: String,
    vars: serde_json::Value,
    enabled: bool,
}

#[derive(Serialize, Debug, Clone)]
struct ChannelResponse {
    id: uuid::Uuid,
    code: String,
    description: String,
    vars: serde_json::Value,
    enabled: bool,
    ws_url: url::Url,
}

async fn create(
    pool: Data<DbPool>,
    path: web::Path<uuid::Uuid>,
    payload: Json<ChannelForm>,
) -> Result<HttpResponse, AppError> {
    let mut conn = pool.get().map_err(AppError::from)?;

    let form = payload.into_inner();
    let ws_id = path.into_inner();

    let channel = Channel::new(
        find_workspace(&mut conn, ws_id)?,
        &form.code,
        &form.description,
        form.vars,
        form.enabled,
    )
    .save(&mut conn)?;

    Ok(HttpResponse::Ok().finish())
}

async fn all() -> Result<HttpResponse, AppError> {
    todo!()
}

async fn find() -> Result<HttpResponse, AppError> {
    todo!()
}

async fn update() -> Result<HttpResponse, AppError> {
    todo!()
}
