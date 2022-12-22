use crate::{
    commons::{json::Json, Id},
    schema::message_types,
};
use diesel::prelude::*;

use super::{channel::Channel, Code};

#[derive(Insertable, Identifiable, Debug, Clone, PartialEq, Queryable, Eq)]
#[diesel(table_name = message_types)]
pub struct MessageType {
    id: Id,
    code: Code,
    description: String,
    vars: Json,
    enabled: bool,
    channel_id: Id,
}

impl MessageType {
    pub fn new(
        id: Id,
        code: Code,
        description: &str,
        vars: Json,
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
