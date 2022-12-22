use crate::{
    commons::{json::Json, Result, Timestamp},
    models::{message::Version, Code},
};
use actix_web::{
    web::{self, post},
    HttpResponse, Scope,
};
use serde::Deserialize;

pub fn routes() -> Scope {
    Scope::new("/messages").service(web::resource("").route(post().to(create)))
}

#[derive(Deserialize, Debug, Clone)]
struct CreateMessageForm {
    r#type: Code,
    version: Version,
    channel: Code,
    payload: Json,
    scheduled_to: Timestamp,
}

async fn create(payload: web::Json<CreateMessageForm>) -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().finish())
}
