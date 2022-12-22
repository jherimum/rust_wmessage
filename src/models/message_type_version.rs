use super::message_type::MessageType;
use super::ModelErrorKind;
use crate::commons::json::{Json, JsonSchema};
use crate::commons::Id;
use crate::schema::message_type_versions::dsl;
use crate::{
    commons::{error::AppError, Result},
    schema::message_type_versions::{self},
};
use diesel::{insert_into, prelude::*};
use uuid::Uuid;

#[derive(Identifiable, Insertable, Debug, Clone, PartialEq, Queryable)]
#[diesel(table_name = message_type_versions)]
pub struct MessageTypeVersion {
    id: Id,
    number: i32,
    schema: Json,
    vars: Json,
    enabled: bool,
    message_type_id: Id,
}

impl MessageTypeVersion {
    pub fn new(
        id: Uuid,
        message_type: &MessageType,
        number: i32,
        schema: JsonSchema,
        vars: Json,
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

    pub fn save(
        conn: &mut PgConnection,
        version: MessageTypeVersion,
    ) -> Result<MessageTypeVersion> {
        match insert_into(dsl::message_type_versions)
            .values(&version)
            .execute(conn)
        {
            Ok(1) => Ok(version),
            Ok(_) => Err(AppError::model_error(ModelErrorKind::EntityNotFound)),
            Err(err) => Err(AppError::from(err)),
        }
    }
}
