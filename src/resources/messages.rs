use crate::{
    commons::{
        error::IntoAppError,
        id::id::new_id,
        types::{Code, DbPool, Json, Result, Timestamp, Version},
    },
    models::{apikey::ApiKey, message::Message, message_type_version::MessageTypeVersion},
};
use actix_web::{
    web::{self, post, Data},
    HttpResponse, Scope,
};
use serde::Deserialize;

pub fn resources() -> Scope {
    Scope::new("/messages").service(web::resource("").route(post().to(create)))
}

#[derive(Deserialize, Debug, Clone)]
struct CreateMessageForm {
    r#type: Code,
    version: Version,
    channel: Code,
    payload: Json,
    scheduled_to: Option<Timestamp>,
}

async fn create(
    payload: web::Json<CreateMessageForm>,
    api_key: ApiKey,
    pool: Data<DbPool>,
) -> Result<HttpResponse> {
    let mut conn = pool.get().into_app_error()?;
    let ws = api_key.workspace(&mut conn)?;

    let version = MessageTypeVersion::find_one(
        &mut conn,
        &ws,
        &payload.channel,
        &payload.r#type,
        &payload.version,
    )?
    .unwrap();

    Message::new(new_id(), &version, &payload.payload, payload.scheduled_to);

    Ok(HttpResponse::Ok().finish())
}
