use actix_web::{
    post,
    web::{self, Data},
    HttpResponse,
};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::{
    app::routes::find_workspace,
    commons::{
        encrypt::argon::Argon,
        error::{AppError, IntoAppError},
    },
    config::DbPool,
    models::apikey::ApiKey,
};
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
) -> Result<HttpResponse, AppError> {
    let mut conn = pool.get().into_app_error()?;

    let ws_id = path.into_inner();
    let form = body.into_inner();

    let ws = find_workspace(&mut conn, ws_id)?;
    let result = ApiKey::new(ws, &form.name, form.ttl, Argon::default())?;
    let tuple = (result.0.save(&mut conn)?, result.1);

    Ok(HttpResponse::Ok().json(ApiKeyResponse::from(tuple)))
}
