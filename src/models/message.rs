use crate::{
    commons::types::{Id, Json, Timestamp},
    schema::messages,
};
use derive_getters::Getters;
use diesel::prelude::*;

use super::message_type_version::MessageTypeVersion;

#[derive(Insertable, Identifiable, Debug, Clone, PartialEq, Queryable, Eq, Getters)]
#[diesel(table_name = messages)]
pub struct Message {
    id: Id,
    workspace_id: Id,
    channel_id: Id,
    message_type_id: Id,
    message_type_version_id: Id,
    payload: Json,
    scheduled_to: Option<Timestamp>,
    status: String,
}

impl Message {
    pub fn new(
        id: Id,
        msg_type_version: &MessageTypeVersion,
        payload: &Json,
        scheduled_to: Option<Timestamp>,
    ) -> Self {
        Message {
            id,
            workspace_id: *msg_type_version.workspace_id(),
            channel_id: *msg_type_version.channel_id(),
            message_type_id: *msg_type_version.message_type_id(),
            message_type_version_id: *msg_type_version.id(),
            payload: payload.clone(),
            scheduled_to,
            status: "RECEIVED".to_string(),
        }
    }
}
