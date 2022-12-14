use crate::commons::Result;
use crate::schema::message_types;
use diesel::prelude::*;
use uuid::Uuid;

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
