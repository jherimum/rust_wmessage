use super::message_type::MessageType;
use crate::commons::Result;
use crate::{
    commons::{json_schema::JsonSchema, uuid::new_uuid},
    schema::message_type_versions,
};
use diesel::prelude::*;
use uuid::Uuid;

#[derive(Identifiable, Insertable, Debug, Clone, PartialEq, Queryable)]
#[diesel(table_name = message_type_versions)]
pub struct MessageTypeVersion {
    id: Uuid,
    number: i32,
    schema: serde_json::Value,
    vars: serde_json::Value,
    enabled: bool,
    message_type_id: Uuid,
}

impl MessageTypeVersion {
    pub fn new(
        message_type: &MessageType,
        number_p: i32,
        schema_p: serde_json::Value,
        vars_p: serde_json::Value,
        enabled_p: bool,
    ) -> Result<Self> {
        match JsonSchema::new(&schema_p) {
            Ok(_) => Ok(MessageTypeVersion {
                id: new_uuid(),
                number: number_p,
                schema: schema_p,
                vars: vars_p,
                enabled: enabled_p,
                message_type_id: *message_type.id(),
            }),
            Err(e) => Err(e),
        }
    }

    pub fn validate(&self, payload: &serde_json::Value) -> Result<Vec<String>> {
        JsonSchema::new(&self.schema)?.validate(payload)
    }
}
