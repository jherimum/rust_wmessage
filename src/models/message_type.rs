use crate::schema::message_types;
use diesel::prelude::*;
use uuid::Uuid;

use super::channel::Channel;

#[derive(Insertable, Identifiable, Debug, Clone, PartialEq, Queryable, Eq)]
#[diesel(table_name = message_types)]
pub struct MessageType {
    id: Uuid,
    code: String,
    description: String,
    vars: serde_json::Value,
    enabled: bool,
    channel_id: Uuid,
}

impl MessageType {
    pub fn new(
        id: Uuid,
        code: &str,
        description: &str,
        vars: serde_json::Value,
        enabled: bool,
        channel: Channel,
    ) -> Self {
        MessageType {
            id: id,
            code: code.trim().to_uppercase(),
            description: description.to_string(),
            vars: vars,
            enabled: enabled,
            channel_id: channel.id().clone(),
        }
    }
}
