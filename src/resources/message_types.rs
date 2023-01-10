use crate::{
    commons::{
        error::{IntoAppError, IntoRestError},
        id::id::new_id,
        rest::entity::{
            IntoSimpleEntity, IntoSimpleEntityCollection, SimpleEntity, SimpleEntityCollection,
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
use std::collections::HashMap;

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

impl IntoSimpleEntity<MessageType> for MessageType {
    fn to_simple_entity(&self, req: &HttpRequest) -> Result<SimpleEntity<MessageType>> {
        Ok(SimpleEntity::new(Some(self.clone()), HashMap::new()))
    }
}

impl IntoSimpleEntityCollection<MessageType> for (Channel, Vec<MessageType>) {
    fn to_simple_entity_collection(
        &self,
        req: &HttpRequest,
    ) -> Result<crate::commons::rest::entity::SimpleEntityCollection<MessageType>> {
        let r: Result<Vec<SimpleEntity<MessageType>>> =
            self.1.iter().map(|mt| mt.to_simple_entity(&req)).collect();

        Ok(SimpleEntityCollection::new(r?, HashMap::new()))
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

    Ok(HttpResponse::Created().json(message_type.to_simple_entity(&req)?))
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
    let mts = MessageType::find_all_by_channel(&mut conn, &channel)?;
    let c = (channel, mts).to_simple_entity_collection(&req)?;
    Ok(HttpResponse::Ok().json(c))
}

async fn find() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().finish())
}
