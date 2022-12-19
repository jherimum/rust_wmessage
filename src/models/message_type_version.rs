use super::message_type::MessageType;
use crate::commons::Result;
use crate::{commons::json_schema::JsonSchema, schema::message_type_versions};
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
        id: Uuid,
        message_type: &MessageType,
        number: i32,
        schema: JsonSchema,
        vars: serde_json::Value,
        enabled: bool,
    ) -> Self {
        MessageTypeVersion {
            id: id,
            number: number,
            schema: schema.raw(),
            vars: vars,
            enabled: enabled,
            message_type_id: message_type.id().clone(),
        }
    }

    pub fn validate(&self, payload: &serde_json::Value) -> Result<Vec<String>> {
        JsonSchema::new(self.schema.clone())?.validate(payload)
    }
}
