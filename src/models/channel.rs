use super::workspace::Workspace;
use crate::commons::error::IntoAppError;
use crate::commons::types::{Code, Id, Json, Result};
use crate::{
    commons::error::AppError,
    schema::channels::{self, dsl},
};
use derive_getters::Getters;
use diesel::{insert_into, prelude::*};
use serde::Serialize;

use uuid::Uuid;

#[derive(Insertable, Identifiable, Debug, Clone, PartialEq, Queryable, Eq, Getters, Serialize)]
#[diesel(table_name = channels)]
pub struct Channel {
    id: Id,
    workspace_id: Id,
    code: Code,
    description: String,
    vars: Json,
    enabled: bool,
}

impl Channel {
    pub fn new(
        id: Uuid,
        ws: &Workspace,
        code: Code,
        description: &str,
        vars: &Json,
        enabled: bool,
    ) -> Channel {
        Channel {
            id,
            workspace_id: *ws.id(),
            code: code.trim().to_uppercase(),
            vars: vars.clone(),
            description: description.to_string(),
            enabled,
        }
    }

    pub fn all_by_workspace(conn: &mut PgConnection, ws: &Workspace) -> Result<Vec<Channel>> {
        channels::table
            .filter(dsl::workspace_id.eq(ws.id()))
            .load::<Channel>(conn)
            .into_app_error()
    }

    pub fn find_by_ws_and_id(
        conn: &mut PgConnection,
        ws_id: &Id,
        id: &Id,
    ) -> Result<Option<Channel>> {
        channels::table
            .filter(dsl::workspace_id.eq(ws_id).and(dsl::id.eq(id)))
            .first::<Channel>(conn)
            .optional()
            .into_app_error()
    }

    pub fn find_by_ws_and_code(
        conn: &mut PgConnection,
        ws: &Workspace,
        code: &Code,
    ) -> Result<Option<Channel>> {
        channels::table
            .filter(dsl::workspace_id.eq(ws.id()).and(dsl::code.eq(code)))
            .first::<Channel>(conn)
            .optional()
            .into_app_error()
    }

    pub fn exists_code(conn: &mut PgConnection, code: &Code) -> Result<bool> {
        channels::table
            .filter(dsl::code.eq(code))
            .count()
            .get_result::<i64>(conn)
            .map(|count| count > 0)
            .into_app_error()
    }

    pub fn save(conn: &mut PgConnection, channel: Channel) -> Result<Channel> {
        if Self::exists_code(conn, channel.code())? {
            return Err(AppError::model_error(
                crate::models::ModelErrorKind::ChannelCodeAlreadyExists {
                    code: channel.code().clone(),
                },
            ));
        }

        match insert_into(dsl::channels).values(&channel).execute(conn) {
            Ok(1) => Ok(channel),
            Ok(_) => Err(AppError::database_error("channel not inserted")),
            Err(err) => Err(AppError::from(err)),
        }
    }
}
