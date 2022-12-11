use chrono::NaiveDateTime;
use uuid::Uuid;

use diesel::prelude::*;

use crate::{
    commons::{error::AppError, uuid::new_uuid},
    schema::api_keys,
};

use super::workspace::Workspace;

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
            id: new_uuid(),
            workspace_id: ws.id(),
            name: name.to_string(),
            prefix: prefix.to_string(),
            hash: hash.to_string(),
            expires_at: *expires_at,
        }
    }

    pub fn workspace(&self, conn: &mut PgConnection) -> Result<Workspace, AppError> {
        match Workspace::find(conn, &self.workspace_id)? {
            Some(ws) => Ok(ws),
            None => Err(AppError::model_error(
                super::ModelErrorKind::EntityNotFound {
                    message: "Worspace not found".to_string(),
                },
            )),
        }
    }
}
