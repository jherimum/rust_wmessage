use super::workspace::Workspace;
use super::IntoEntityNotFound;
use crate::commons::error::IntoAppError;
use crate::commons::id::id::new_id;
use crate::commons::types::{Id, Result, Timestamp};
use crate::commons::{encrypt::Encrypter, error::AppError};
use crate::schema::api_keys;
use actix_web::FromRequest;
use chrono::{Duration, Utc};
use derive_getters::Getters;
use diesel::{insert_into, prelude::*};
use std::future::{ready, Ready};

#[derive(Insertable, Queryable, Identifiable, Debug, Clone, Getters)]
#[diesel(table_name = api_keys)]
pub struct ApiKey {
    id: Id,
    workspace_id: Id,
    name: String,
    hash: String,
    expires_at: Timestamp,
}

impl ApiKey {
    pub fn new(
        ws: Workspace,
        name: &str,
        ttl: u8,
        encrypter: impl Encrypter,
    ) -> Result<(ApiKey, String)> {
        let _id = new_id();
        let key = new_id();
        Ok((
            ApiKey {
                id: _id,
                workspace_id: *ws.id(),
                name: name.to_string(),
                hash: encrypter.encrypt(&key.to_string())?,
                expires_at: (Utc::now() + Duration::days(ttl as i64)).naive_utc(),
            },
            format!("{}.{}", _id, key),
        ))
    }

    pub fn save(conn: &mut PgConnection, api_key: ApiKey) -> Result<ApiKey> {
        match insert_into(api_keys::table).values(&api_key).execute(conn) {
            Ok(1) => Ok(api_key),
            Ok(_) => Err(AppError::database_error("apikey not inserted")),
            Err(err) => Err(AppError::from(err)),
        }
    }

    pub fn workspace(self, conn: &mut PgConnection) -> Result<Workspace> {
        Workspace::find(conn, self.id())
            .into_app_error()?
            .into_entity_not_found(&format!("workspace with id {} not found", self.id()))
    }
}

impl FromRequest for ApiKey {
    type Error = AppError;

    type Future = Ready<Result<ApiKey>>;

    fn from_request(
        req: &actix_web::HttpRequest,
        payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        let key = req.headers().get("x-api-key");
        ready(Err(AppError::new(
            crate::commons::error::AppErrorKind::UnexpectedError,
            "nao achou api keu",
            None,
        )))
    }
}
