use crate::commons::{error::AppError, Result};
use crate::models::message_type_version::MessageTypeVersion;
use crate::models::ModelErrorKind;
use crate::schema::message_type_versions::dsl::*;

use diesel::{insert_into, prelude::*};
pub struct MessageTypeVersions;

impl MessageTypeVersions {
    pub fn save(
        conn: &mut PgConnection,
        version: MessageTypeVersion,
    ) -> Result<MessageTypeVersion> {
        match insert_into(message_type_versions)
            .values(&version)
            .execute(conn)
        {
            Ok(1) => Ok(version),
            Ok(_) => Err(AppError::model_error(ModelErrorKind::EntityNotFound)),
            Err(err) => Err(AppError::from(err)),
        }
    }
}
