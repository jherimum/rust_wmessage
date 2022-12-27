use super::message_type::MessageType;
use super::ModelErrorKind;
use crate::commons::error::IntoAppError;
use crate::commons::json::JsonSchema;
use crate::commons::types::{Code, Id, Json, Result, Version};
use crate::schema::message_type_versions::dsl;
use crate::schema::workspaces;
use crate::schema::{channels, message_types};
use crate::{
    commons::error::AppError,
    schema::message_type_versions::{self},
};
use derive_getters::Getters;
use diesel::{insert_into, prelude::*};
use uuid::Uuid;

#[derive(Identifiable, Insertable, Debug, Clone, PartialEq, Eq, Queryable, Getters)]
#[diesel(table_name = message_type_versions)]
pub struct MessageTypeVersion {
    id: Id,
    number: Version,
    schema: Json,
    vars: Json,
    enabled: bool,
    message_type_id: Id,
    channel_id: Id,
    workspace_id: Id,
}

impl MessageTypeVersion {
    pub fn find() -> Result<Option<MessageTypeVersion>> {
        Ok(None)
    }

    pub fn find_one(
        conn: &mut PgConnection,
        ws_code: &Code,
        channel_code: &Code,
        message_type_code: &Code,
        message_type_version: &Version,
    ) -> Result<Option<MessageTypeVersion>> {
        message_type_versions::table
            .inner_join(
                message_types::table.inner_join(channels::table.inner_join(workspaces::table)),
            )
            .filter(
                workspaces::code
                    .eq(ws_code)
                    .and(channels::code.eq(channel_code))
                    .and(message_types::code.eq(message_type_code))
                    .and(message_type_versions::number.eq(message_type_version)),
            )
            .select(message_type_versions::table::all_columns())
            .first::<MessageTypeVersion>(conn)
            .optional()
            .into_app_error()
    }

    pub fn new(
        id: Uuid,
        message_type: &MessageType,
        number: i32,
        schema: JsonSchema,
        vars: Json,
        enabled: bool,
    ) -> Self {
        MessageTypeVersion {
            id,
            number,
            schema: schema.raw(),
            vars,
            enabled,
            message_type_id: *message_type.id(),
            channel_id: *message_type.channel_id(),
            workspace_id: *message_type.workspace_id(),
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
