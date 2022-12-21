use crate::commons::database::DbPool;
use crate::commons::error::IntoRestError;
use crate::commons::mock_uuid::new_uuid;
use crate::commons::Result;
use crate::models::workspace::Workspace;
use crate::{commons::error::IntoAppError, models::channel::Channel};
use actix_web::HttpRequest;
use actix_web::{
    web::{self, get, patch, post, Data, Json},
    HttpResponse, Scope,
};
use serde::{Deserialize, Serialize};
use url::Url;

pub fn routes() -> Scope {
    let channels = web::resource("")
        .route(post().to(create))
        .route(get().to(all))
        .name("channel");
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
    self_url: Url,
}

fn to_response(channel: Channel, req: HttpRequest) -> Result<ChannelResponse> {
    Ok(ChannelResponse {
        id: channel.id().clone(),
        code: channel.code().clone(),
        description: channel.description().clone(),
        vars: channel.vars().clone(),
        enabled: channel.enabled().clone(),
        self_url: req.url_for(
            "channel",
            &[
                channel.workspace_id().to_string(),
                channel.id().clone().to_string(),
            ],
        )?,
    })
}

async fn create(
    pool: Data<DbPool>,
    path: web::Path<uuid::Uuid>,
    payload: Json<ChannelForm>,
    req: HttpRequest,
) -> Result<HttpResponse> {
    let mut conn = pool.get().into_app_error()?;

    let form = payload.into_inner();
    let ws_id = path.into_inner();

    let workspace = Workspace::find(&mut conn, &ws_id).into_not_found("Workspace not found")?;

    let channel = Channel::new(
        new_uuid(),
        workspace,
        &form.code,
        &form.description,
        form.vars,
        form.enabled,
    );
    let channel = Channel::save(&mut conn, channel)?;

    Ok(HttpResponse::Created().json(to_response(channel, req)?))
}

async fn all() -> Result<HttpResponse> {
    todo!()
}

async fn find() -> Result<HttpResponse> {
    todo!()
}

async fn update() -> Result<HttpResponse> {
    todo!()
}
