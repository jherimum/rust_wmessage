use actix_web::{
    post,
    web::{self, Data},
    HttpResponse, Responder,
};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::app::State;

use uuid::Uuid;

#[derive(Deserialize, Debug, Clone)]
struct ApiKeyForm {
    name: String,
    ttl: u8,
}

#[derive(Serialize, Debug)]
struct ApiKeyResponse {
    id: Uuid,
    name: String,
    prefix: String,
    expires_at: NaiveDateTime,
    key: Option<String>,
}

#[post("/api/worspaces/{ws_id}/apikeys")]
pub async fn register(
    appState: Data<State>,
    body: web::Json<ApiKeyForm>,
    path: web::Path<Uuid>,
) -> impl Responder {
    let (ws_id) = path.into_inner();
    let form = body.into_inner();
    let api_key = uuid::Uuid::new_v4().to_string();

    HttpResponse::Ok().finish()
}
