use crate::{
    commons::{error::AppError, json_schema::JsonSchema, new_uuid},
    schema::message_type_versions,
};
use diesel::{insert_into, prelude::*};

use uuid::Uuid;

use super::message_type::MessageType;

use crate::schema::message_type_versions::dsl::*;

#[derive(Identifiable, Insertable, Debug, Clone, PartialEq, Queryable)]
#[diesel(table_name = message_type_versions)]
struct MessageTypeVersion {
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
    ) -> Result<Self, AppError> {
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

    pub fn save(&self, conn: &mut PgConnection) -> Result<MessageTypeVersion, AppError> {
        match insert_into(message_type_versions)
            .values(self)
            .execute(conn)
        {
            Ok(1) => Ok(self.clone()),
            Ok(_) => Err(AppError::model_error(
                super::ModelErrorKind::EntityNotFound {
                    message: "message type version not inserted".to_string(),
                },
            )),
            Err(err) => Err(AppError::from(err)),
        }
    }

    pub fn validate(&self, payload: &serde_json::Value) -> Result<Vec<String>, AppError> {
        JsonSchema::new(&self.schema)?.validate(payload)
    }
}
