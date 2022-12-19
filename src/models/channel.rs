use super::workspace::Workspace;
use crate::schema::channels;
use derive_getters::Getters;
use diesel::prelude::*;
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
}
