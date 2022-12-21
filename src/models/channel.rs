use super::workspace::Workspace;
use crate::commons::error::IntoAppError;
use crate::{
    commons::{error::AppError, Result},
    schema::channels::{self, dsl},
};
use derive_getters::Getters;
use diesel::{insert_into, prelude::*};
use serde_json::Value;
use uuid::Uuid;

#[derive(Insertable, Identifiable, Debug, Clone, PartialEq, Queryable, Eq, Getters)]
#[diesel(table_name = channels)]
pub struct Channel {
    id: Uuid,
    workspace_id: Uuid,
    code: String,
    description: String,
    vars: Value,
    enabled: bool,
}

impl Channel {
    pub fn new(
        id: Uuid,
        ws: Workspace,
        code: &str,
        description: &str,
        vars: Value,
        enabled: bool,
    ) -> Channel {
        Channel {
            id: id,
            workspace_id: ws.id().clone(),
            code: code.trim().to_uppercase(),
            vars: vars,
            description: description.to_string(),
            enabled: enabled,
        }
    }

    pub fn exists_code(conn: &mut PgConnection, _code: &str) -> Result<bool> {
        channels::table
            .filter(dsl::code.eq(_code))
            .count()
            .get_result::<i64>(conn)
            .map(|count| count > 0)
            .into_app_error()
    }

    pub fn save(conn: &mut PgConnection, channel: Channel) -> Result<Channel> {
        if Self::exists_code(conn, &channel.code())? {
            return Err(AppError::model_error(
                crate::models::ModelErrorKind::ChannelCodeAlreadyExists {
                    code: channel.code().clone(),
                },
            ));
        }

        match insert_into(dsl::channels).values(&channel).execute(conn) {
            Ok(1) => Ok(channel),
            Ok(_) => Err(AppError::database_error("channel not inserted")),
            Err(err) => Err(AppError::from(err)),
        }
    }
}
