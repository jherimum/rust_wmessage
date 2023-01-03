use super::Resource;
use crate::commons::error::IntoRestError;
use crate::commons::id::id::new_id;
use crate::commons::rest::entity::{EntityModel, IntoEntityModel};
use crate::commons::rest::link::{IntoLinks, Link, SELF_ID};
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

impl IntoEntityModel<Vec<EntityModel<Channel>>> for (Workspace, Vec<Channel>) {
    fn to_entity_model(&self, req: &HttpRequest) -> Result<EntityModel<Vec<EntityModel<Channel>>>> {
        let channels = &self.1;
        let entities = channels
            .into_iter()
            .map(|c| c.to_entity_model(req).unwrap()) //Deveria aceitar ?
            .collect();

        Ok(EntityModel::new()
            .with_data(entities)
            .with_link(
                Resource::Channels {
                    ws_id: *self.0.id(),
                }
                .link(SELF_ID, &req)?,
            )
            .clone())
    }
}

impl IntoEntityModel<Channel> for Channel {
    fn to_entity_model(&self, req: &HttpRequest) -> Result<EntityModel<Channel>> {
        Ok(EntityModel::new()
            .with_data(self.clone())
            .with_links(self.to_links(&req)?)
            .clone())
    }
}

impl IntoLinks for Channel {
    fn to_links(&self, req: &HttpRequest) -> Result<Vec<Link>> {
        let mut vec = vec![];
        vec.push(
            Resource::Channel {
                ws_id: *self.workspace_id(),
                channel_id: *self.id(),
            }
            .link(SELF_ID, req)?,
        );
        vec.push(
            Resource::Channels {
                ws_id: *self.workspace_id(),
            }
            .link("channels", req)?,
        );
        Ok(vec)
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
    let workspace = retrieve_workspace(&mut conn, &path.into_inner())?;
    let channel = build_channel(&workspace, &payload);
    let channel = Channel::save(&mut conn, channel)?;

    channel.to_entity_model(&req)?.created(Some(
        Resource::Channel {
            ws_id: *channel.workspace_id(),
            channel_id: *channel.id(),
        }
        .url(&req)?,
    ))
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

pub async fn all_channels(
    pool: Data<DbPool>,
    path: web::Path<Id>,
    req: HttpRequest,
) -> Result<HttpResponse> {
    let mut conn = pool.get().into_app_error()?;
    let ws = retrieve_workspace(&mut conn, &path.into_inner())?;
    let channels = Channel::all_by_workspace(&mut conn, &ws)?;
    (ws, channels).to_entity_model(&req)?.ok()
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
