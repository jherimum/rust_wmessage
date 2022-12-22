use crate::{
    commons::{json::Json, Id, Timestamp},
    schema::messages,
};
use derive_getters::Getters;
use diesel::prelude::*;

pub type Version = u8;

#[derive(Insertable, Identifiable, Debug, Clone, PartialEq, Queryable, Eq, Getters)]
#[diesel(table_name = messages)]
pub struct Message {
    id: Id,
    workspace_id: Id,
    channel_id: Id,
    message_type_version_id: Id,
    payload: Json,
    scheduled_to: Option<Timestamp>,
    status: String,
}

impl Message {}
