use crate::{
    commons::{
        error::{AppError, IntoAppError},
        types::{Code, Id, Json, Result},
    },
    schema::message_types::{self, dsl},
};
use derive_getters::Getters;
use diesel::{insert_into, prelude::*};
use serde::Serialize;

use super::channel::Channel;

#[derive(Insertable, Identifiable, Debug, Clone, PartialEq, Queryable, Eq, Getters, Serialize)]
#[diesel(table_name = message_types)]
pub struct MessageType {
    id: Id,
    code: Code,
    description: String,
    vars: Json,
    enabled: bool,
    channel_id: Id,
    workspace_id: Id,
}

impl MessageType {
    pub fn new(
        id: Id,
        code: &Code,
        description: &str,
        vars: &Json,
        enabled: &bool,
        channel: &Channel,
    ) -> Self {
        MessageType {
            id,
            code: code.trim().to_uppercase(),
            description: description.to_string(),
            vars: vars.clone(),
            enabled: *enabled,
            channel_id: *channel.id(),
            workspace_id: *channel.workspace_id(),
        }
    }

    pub fn find_by_channel_and_code(
        conn: &mut PgConnection,
        channel: &Channel,
        code: &Code,
    ) -> Result<Option<Self>> {
        message_types::table
            .filter(dsl::channel_id.eq(channel.id()).and(dsl::code.eq(code)))
            .first::<MessageType>(conn)
            .optional()
            .into_app_error()
    }

    pub fn save(conn: &mut PgConnection, message_type: Self) -> Result<Self> {
        match insert_into(dsl::message_types)
            .values(&message_type)
            .execute(conn)
        {
            Ok(1) => Ok(message_type),
            Ok(_) => Err(AppError::database_error("channel not inserted")),
            Err(err) => Err(AppError::from(err)),
        }
    }

    pub fn find_all_by_channel(conn: &mut PgConnection, channel: &Channel) -> Result<Vec<Self>> {
        message_types::table
            .filter(dsl::channel_id.eq(channel.id()))
            .load(conn)
            .into_app_error()
    }
}
