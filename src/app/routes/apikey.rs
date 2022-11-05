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
    app::State,
    models::{apikey::ApiKey, workspace::Workspace},
};
use rand::distributions::DistString;
use uuid::Uuid;

#[derive(Deserialize, Debug, Clone)]
struct ApiKeyForm {
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
    appState: Data<State>,
    body: web::Json<ApiKeyForm>,
    path: web::Path<Uuid>,
) -> impl Responder {
    let mut conn = appState.pool.get().expect("msg");

    let ws_id = path.into_inner();
    let form = body.into_inner();

    let ws = match Workspace::find(&mut conn, &ws_id) {
        Err(e) => return HttpResponse::InternalServerError(),
        Ok(None) => {
            return HttpResponse::NotFound();
        }
        Ok(Some(ws)) => ws,
    };

    let api_key = uuid::Uuid::new_v4().to_string();
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
