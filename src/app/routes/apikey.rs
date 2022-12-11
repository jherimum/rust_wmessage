use actix_web::{
    post,
    web::{self, Data},
    HttpResponse, Responder,
};
use bcrypt::DEFAULT_COST;
use chrono::{Duration, NaiveDateTime, Utc};
use rand::distributions::Alphanumeric;
use serde::{Deserialize, Serialize};

use crate::{
    commons::mock_uuid::new_uuid,
    config::DbPool,
    models::{apikey::ApiKey, workspace::Workspace},
};
use rand::distributions::DistString;
use uuid::Uuid;

#[derive(Deserialize, Debug, Clone)]
pub struct ApiKeyForm {
    name: String,
    ttl: i64,
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
    pool: Data<DbPool>,
    body: web::Json<ApiKeyForm>,
    path: web::Path<Uuid>,
) -> impl Responder {
    let mut conn = pool.get().expect("msg");

    let ws_id = path.into_inner();
    let form = body.into_inner();

    let ws = match Workspace::find(&mut conn, &ws_id) {
        Err(_) => return HttpResponse::InternalServerError(),
        Ok(None) => {
            return HttpResponse::NotFound();
        }
        Ok(Some(ws)) => ws,
    };

    let api_key = new_uuid().to_string();
    let prefix = Alphanumeric.sample_string(&mut rand::thread_rng(), 16);
    let hash = bcrypt::hash(api_key, DEFAULT_COST).expect("msg");

    ApiKey::create(
        &ws,
        &form.name,
        &prefix,
        &hash,
        &(Utc::now() + Duration::days(form.ttl)).naive_utc(),
    );

    HttpResponse::Ok()
}
