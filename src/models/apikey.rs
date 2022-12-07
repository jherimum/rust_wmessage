use chrono::NaiveDateTime;
use uuid::Uuid;

use diesel::prelude::*;

use crate::schema::api_keys;

use super::workspace::Workspace;
use anyhow::bail;

#[derive(Insertable, Queryable, Identifiable, Debug, Clone)]
#[diesel(table_name = api_keys)]
pub struct ApiKey {
    id: Uuid,
    workspace_id: Uuid,
    name: String,
    prefix: String,
    hash: String,
    expires_at: NaiveDateTime,
}

impl ApiKey {
    pub fn create(
        ws: &Workspace,
        name: &String,
        prefix: &String,
        hash: &String,
        expires_at: &NaiveDateTime,
    ) -> ApiKey {
        ApiKey {
            id: Uuid::new_v4(),
            workspace_id: ws.id(),
            name: name.to_string(),
            prefix: prefix.to_string(),
            hash: hash.to_string(),
            expires_at: expires_at.clone(),
        }
    }

    pub fn workspace(&self, conn: &mut PgConnection) -> anyhow::Result<Workspace> {
        match Workspace::find(conn, &self.workspace_id)? {
            Some(ws) => Ok(ws),
            None => bail!("workspace not found"),
        }
    }
}
