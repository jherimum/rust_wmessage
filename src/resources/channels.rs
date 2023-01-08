use super::{AsUrl, Resource};
use crate::commons::error::IntoRestError;
use crate::commons::id::id::new_id;
use crate::commons::rest::entity::{
    CollectionModel, Entity, EntityModel, ToCollectionModel, ToEntityModel,
};
use crate::commons::rest::link::SELF_ID;
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

pub const CHANNEL_RESOURCE: &str = "channel";
pub const CHANNELS_RESOURCE: &str = "channels";

impl ToCollectionModel<Channel> for (Workspace, Vec<Channel>) {
    fn to_collection_model(&self, req: &HttpRequest) -> Result<CollectionModel<Channel>> {
        Ok(CollectionModel::new()
            .add_to_entities(&self.1, req)?
            .with_link(
                req,
                SELF_ID,
                Resource::Channels {
                    ws_id: *self.0.id(),
                },
            )?
            .clone())
    }
}

impl ToEntityModel<Channel> for Channel {
    fn to_entity_model(&self, req: &HttpRequest) -> Result<EntityModel<Channel>> {
        Ok(EntityModel::new()
            .with_data(self.clone())
            .with_link(
                req,
                SELF_ID,
                Resource::Channel {
                    ws_id: *self.workspace_id(),
                    channel_id: *self.id(),
                },
            )?
            .with_link(
                req,
                "channels",
                Resource::Channels {
                    ws_id: *self.workspace_id(),
                },
            )?
            .with_link(
                req,
                "messageTypes",
                Resource::MessageTypes {
                    ws_id: *self.workspace_id(),
                    channel_id: *self.id(),
                },
            )?
            .clone())
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
        .name(CHANNELS_RESOURCE)
        .route(post().to(create_channel))
        .route(get().to(all_channels));

    let channel = web::resource("/{channel_id}")
        .name(CHANNEL_RESOURCE)
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
        .to_url(&req)?,
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
    (ws, channels).to_collection_model(&req)?.ok()
}

pub async fn find_channel(
    pool: Data<DbPool>,
    path: web::Path<(Id, Id)>,
    req: HttpRequest,
) -> Result<HttpResponse> {
    let mut conn = pool.get().into_app_error()?;
    Channel::find_by_ws_and_id(&mut conn, &path.0, &path.1)
        .into_not_found("Channel not found")?
        .to_entity_model(&req)?
        .ok()
}

pub async fn update_channel() -> Result<HttpResponse> {
    todo!()
}

pub async fn delete_channel() -> Result<HttpResponse> {
    todo!()
}
