use crate::{
    commons::{
        error::IntoAppError,
        types::{Code, Id, Json, Result},
    },
    schema::message_types::{self, dsl},
};
use derive_getters::Getters;
use diesel::prelude::*;

use super::channel::Channel;

#[derive(Insertable, Identifiable, Debug, Clone, PartialEq, Queryable, Eq, Getters)]
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
        enabled: bool,
        channel: &Channel,
    ) -> Self {
        MessageType {
            id,
            code: code.trim().to_uppercase(),
            description: description.to_string(),
            vars: vars.clone(),
            enabled,
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
}
