use crate::commons::error::AppError;
use crate::commons::uuid::new_uuid;
use crate::schema::{self, channels};
use diesel::prelude::*;
use diesel::{insert_into, PgConnection};
use schema::channels::dsl::*;
use serde_json::Value;
use uuid::Uuid;

use super::workspace::Workspace;

#[derive(Insertable, Identifiable, Debug, Clone, PartialEq, Queryable, Eq)]
#[diesel(table_name = channels)]
pub struct Channel {
    id: Uuid,
    workspace_id: Uuid,
    code: String,
    description: String,
    vars: Value,
    enabled: bool,
}

impl Channel {
    pub fn new(
        ws: Workspace,
        _code: String,
        _desc: String,
        _vars: Value,
        _enabled: bool,
    ) -> Channel {
        Channel {
            id: new_uuid(),
            workspace_id: ws.id(),
            code: _code.to_uppercase(),
            vars: _vars,
            description: _desc,
            enabled: _enabled,
        }
    }

    pub fn exists_code(conn: &mut PgConnection, _code: &str) -> Result<bool, AppError> {
        channels::table
            .filter(code.eq(_code))
            .count()
            .get_result::<i64>(conn)
            .map(|count| count > 0)
            .map_err(AppError::from)
    }

    pub fn save(self, conn: &mut PgConnection) -> Result<Channel, AppError> {
        if Self::exists_code(conn, &self.code)? {
            return Err(AppError::model_error(
                crate::models::ModelErrorKind::ChannelCodeAlreadyExists { code: self.code },
            ));
        }

        match insert_into(channels).values(&self).execute(conn) {
            Ok(1) => Ok(self),
            Ok(_) => Err(AppError::database_error("channel not inserted")),
            Err(err) => Err(AppError::from(err)),
        }
    }
}
