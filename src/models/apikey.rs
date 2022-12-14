use crate::commons::encrypt::Encrypter;
use crate::commons::error::AppError;
use crate::commons::uuid::new_uuid;
use crate::commons::Result;
use chrono::{Duration, NaiveDateTime, Utc};
use derive_getters::Getters;
use diesel::prelude::*;
use diesel::{insert_into, PgConnection};
use uuid::Uuid;

use crate::schema::api_keys;

use super::workspace::Workspace;

#[derive(Insertable, Queryable, Identifiable, Debug, Clone, Getters)]
#[diesel(table_name = api_keys)]
pub struct ApiKey {
    id: Uuid,
    workspace_id: Uuid,
    name: String,
    hash: String,
    expires_at: NaiveDateTime,
}

impl ApiKey {
    pub fn new(
        ws: Workspace,
        name: &str,
        ttl: u8,
        encrypter: impl Encrypter,
    ) -> Result<(ApiKey, String)> {
        let _id = new_uuid();
        let key = new_uuid();
        Ok((
            ApiKey {
                id: _id,
                workspace_id: ws.id().clone(),
                name: name.to_string(),
                hash: encrypter.encrypt(&key.to_string())?,
                expires_at: (Utc::now() + Duration::days(ttl as i64)).naive_utc(),
            },
            format!("{}.{}", _id, key),
        ))
    }

    pub fn save(&self, conn: &mut PgConnection) -> Result<ApiKey> {
        match insert_into(api_keys::table).values(self).execute(conn) {
            Ok(1) => Ok(self.clone()),
            Ok(_) => Err(AppError::database_error("apikey not inserted")),
            Err(err) => Err(AppError::from(err)),
        }
    }

    pub fn workspace(&self, conn: &mut PgConnection) -> Result<Workspace> {
        match Workspace::find(conn, &self.workspace_id)? {
            Some(ws) => Ok(ws),
            None => Err(AppError::model_error(super::ModelErrorKind::EntityNotFound)),
        }
    }
}
