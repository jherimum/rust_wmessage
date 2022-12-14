use super::workspace::Workspace;
use crate::commons::encrypt::Encrypter;
use crate::commons::uuid::new_uuid;
use crate::commons::Result;
use crate::schema::api_keys;
use chrono::{Duration, NaiveDateTime, Utc};
use derive_getters::Getters;
use diesel::prelude::*;
use uuid::Uuid;

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
}
