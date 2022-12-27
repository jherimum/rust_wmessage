use crate::commons::error::IntoRestError;
use crate::commons::id::id::new_id;
use crate::commons::rest::entity::{AsResponse, EntityModel};
use crate::commons::rest::link::{IntoLinks, Links, SELF_ID};
use crate::commons::types::{Code, Conn, DbPool, Id, Json, Result};
use crate::models::workspace::Workspace;
use crate::{commons::error::IntoAppError, models::channel::Channel};
use actix_web::web::{delete, patch};
use actix_web::{
    web::{self, get, post, Data},
    HttpResponse,
};
use actix_web::{HttpRequest, Scope};
use serde::Deserialize;

use super::ResourceLink;

impl AsResponse for Channel {
    type T = Channel;

    fn to_response(&self, req: &HttpRequest) -> Result<EntityModel<Self::T>> {
        let links = self.to_links(&req)?;
        EntityModel::new(Some(self.clone()), links)
    }
}

impl IntoLinks for Channel {
    fn to_links(&self, req: &HttpRequest) -> Result<Links> {
        let mut vec = vec![];
        vec.push(
            ResourceLink::Channel {
                ws_id: *self.workspace_id(),
                channel_id: *self.id(),
            }
            .link(SELF_ID, req)?,
        );
        vec.push(
            ResourceLink::Channels {
                ws_id: *self.workspace_id(),
            }
            .link("channels", req)?,
        );
        Ok(Links::new(vec))
    }
}

#[derive(Deserialize, Debug, Clone)]
struct CreateChannel {
    code: Code,
    description: String,
    vars: Json,
    enabled: bool,
}

pub fn resources() -> Scope {
    let channels = web::resource("")
        .name("channels")
        .route(post().to(create_channel))
        .route(get().to(all_channels));

    let channel = web::resource("/{channel_id}")
        .name("channel")
        .route(get().to(find_channel))
        .route(patch().to(update_channel))
        .route(delete().to(delete_channel));

    Scope::new("/workspaces/{ws_id}/channels")
        .service(channel)
        .service(channels)
}

async fn create_channel(
    pool: Data<DbPool>,
    path: web::Path<Id>,
    payload: web::Json<CreateChannel>,
    req: HttpRequest,
) -> Result<HttpResponse> {
    let mut conn = pool.get().into_app_error()?;
    let ws_id = path.into_inner();
    let workspace = retrieve_workspace(&mut conn, &ws_id)?;
    let channel = build_channel(&workspace, &payload);
    let channel = Channel::save(&mut conn, channel)?;

    channel.to_response(&req)?.created(
        ResourceLink::Channel {
            ws_id: ws_id,
            channel_id: *channel.id(),
        }
        .url(&req)
        .ok(),
    )
}

fn retrieve_workspace(conn: &mut Conn, ws_id: &Id) -> Result<Workspace> {
    Workspace::find(conn, ws_id).into_not_found("Workspace not found")
}

fn build_channel(workspace: &Workspace, payload: &web::Json<CreateChannel>) -> Channel {
    Channel::new(
        new_id(),
        workspace,
        payload.code.to_owned(),
        &payload.description,
        &payload.vars,
        payload.enabled,
    )
}

pub async fn all_channels() -> Result<HttpResponse> {
    todo!()
}

pub async fn find_channel() -> Result<HttpResponse> {
    todo!()
}

pub async fn update_channel() -> Result<HttpResponse> {
    todo!()
}

pub async fn delete_channel() -> Result<HttpResponse> {
    todo!()
}
