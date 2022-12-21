use crate::commons::{database::DbPool, Result};
use crate::models::workspace::Workspace;
use crate::{
    commons::{encrypt::argon::Argon, error::IntoAppError, error::IntoRestError},
    models::apikey::ApiKey,
};
use actix_web::{
    post,
    web::{self, Data},
    HttpResponse,
};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Debug, Clone)]
pub struct ApiKeyForm {
    name: String,
    ttl: u8,
}

#[derive(Serialize, Debug)]
struct ApiKeyResponse {
    id: Uuid,
    name: String,
    expires_at: NaiveDateTime,
    key: Option<String>,
}

impl From<(ApiKey, String)> for ApiKeyResponse {
    fn from(k: (ApiKey, String)) -> Self {
        ApiKeyResponse {
            id: k.0.id().clone(),
            name: k.0.name().clone(),
            expires_at: k.0.expires_at().clone(),
            key: Some(k.1),
        }
    }
}

#[post("/worspaces/{ws_id}/apikeys")]
pub async fn create(
    pool: Data<DbPool>,
    body: web::Json<ApiKeyForm>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse> {
    let mut conn = pool.get().into_app_error()?;

    let ws_id = path.into_inner();
    let form = body.into_inner();

    let ws = Workspace::find(&mut conn, &ws_id).into_not_found("Worspace not found")?;
    let result = ApiKey::new(ws, &form.name, form.ttl, Argon::default())?;
    let tuple = (ApiKey::save(&mut conn, result.0)?, result.1);

    Ok(HttpResponse::Ok().json(ApiKeyResponse::from(tuple)))
}
