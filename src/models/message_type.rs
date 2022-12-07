use crate::schema::{self, message_types};
use diesel::prelude::*;
use uuid::Uuid;

use diesel::OptionalExtension;
use schema::message_types::dsl::*;

#[derive(Insertable, Identifiable, Debug, Clone, PartialEq, Queryable)]
#[diesel(table_name = message_types)]
struct MessageType {
    id: Uuid,
    code: String,
    description: String,
    vars: serde_json::Value,
    enabled: bool,
    channel_id: Uuid,
}
