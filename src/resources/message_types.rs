use super::{AsUrl, Resource};
use crate::{
    commons::{
        error::{IntoAppError, IntoRestError},
        id::id::new_id,
        rest::{
            entity::{CollectionModel, Entity, EntityModel, ToCollectionModel, ToEntityModel},
            link::SELF_ID,
        },
        types::{Code, DbPool, Id, Json, Result},
    },
    models::{channel::Channel, message_type::MessageType},
};
use actix_web::{
    web::{self, get, post, Data},
    HttpRequest, HttpResponse, Scope,
};
use diesel::PgConnection;
use serde::Deserialize;

pub const MESSAGE_TYPES_RESOURCE: &str = "message_types";
pub const MESSAGE_TYPE_RESOURCE: &str = "message_type";

pub fn routes() -> Scope {
    let message_types = web::resource("")
        .name(MESSAGE_TYPES_RESOURCE)
        .route(post().to(create))
        .route(get().to(all));

    let message_type = web::resource("/{message_type_id}")
        .name(MESSAGE_TYPE_RESOURCE)
        .route(get().to(find));

    Scope::new("/workspaces/{ws_id}/channels/{channel_id}/message_types")
        .service(message_type)
        .service(message_types)
}

impl ToEntityModel<MessageType> for MessageType {
    fn to_entity_model(&self, req: &HttpRequest) -> Result<EntityModel<MessageType>> {
        Ok(EntityModel::new()
            .with_data(self.clone())
            .with_link(
                req,
                "messageTypes",
                Resource::MessageTypes {
                    ws_id: *self.workspace_id(),
                    channel_id: *self.channel_id(),
                },
            )?
            .with_link(
                req,
                SELF_ID,
                Resource::MessageType {
                    ws_id: *self.workspace_id(),
                    channel_id: *self.channel_id(),
                    message_type_id: *self.id(),
                },
            )?
            .clone())
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct CreateMessageType {
    code: Code,
    description: String,
    vars: Json,
    enabled: bool,
}

async fn create(
    pool: Data<DbPool>,
    path: web::Path<(Id, Id)>,
    payload: web::Json<CreateMessageType>,
    req: HttpRequest,
) -> Result<HttpResponse> {
    let mut conn = pool.get().into_app_error()?;
    let channel = retrieve_channel(&mut conn, path)?;

    let message_type = MessageType::new(
        new_id(),
        &payload.code,
        &payload.description,
        &payload.vars,
        &payload.enabled,
        &channel,
    );

    let message_type = MessageType::save(&mut conn, message_type)?;

    message_type.to_entity_model(&req)?.created(Some(
        Resource::MessageType {
            ws_id: *message_type.workspace_id(),
            channel_id: *message_type.channel_id(),
            message_type_id: *message_type.id(),
        }
        .to_url(&req)?,
    ))
}

fn retrieve_channel(conn: &mut PgConnection, path: web::Path<(Id, Id)>) -> Result<Channel> {
    Channel::find_by_ws_and_id(conn, &path.0, &path.1).into_not_found("message")
}

async fn all(
    pool: Data<DbPool>,
    path: web::Path<(Id, Id)>,
    req: HttpRequest,
) -> Result<HttpResponse> {
    let mut conn = pool.get().into_app_error()?;
    let channel = retrieve_channel(&mut conn, path)?;

    (
        &channel,
        &MessageType::find_all_by_channel(&mut conn, &channel)?,
    )
        .to_collection_model(&req)?
        .ok()
}

impl ToCollectionModel<MessageType> for (&Channel, &Vec<MessageType>) {
    fn to_collection_model(&self, req: &HttpRequest) -> Result<CollectionModel<MessageType>> {
        Ok(CollectionModel::new()
            .add_to_entities(self.1, &req)?
            .with_link(
                req,
                "channel",
                Resource::Channel {
                    ws_id: *self.0.workspace_id(),
                    channel_id: *self.0.id(),
                },
            )?
            .with_link(
                req,
                SELF_ID,
                Resource::MessageTypes {
                    ws_id: *self.0.workspace_id(),
                    channel_id: *self.0.id(),
                },
            )?
            .clone())
    }
}

async fn find() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().finish())
}
