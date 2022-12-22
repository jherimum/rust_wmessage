use super::workspace::Workspace;
use crate::commons::id::Id::new_id;
use crate::commons::{encrypt::Encrypter, error::AppError};
use crate::commons::{Id, Result, Timestamp};
use crate::schema::api_keys;
use chrono::{Duration, Utc};
use derive_getters::Getters;
use diesel::{insert_into, prelude::*};

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
                workspace_id: ws.id().clone(),
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
}
