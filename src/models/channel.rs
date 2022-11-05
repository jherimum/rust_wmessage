use diesel::PgConnection;
use serde_json::json;
use serde_json::Value as Json;
use uuid::Uuid;

use super::workspace::Workspace;
use anyhow::Result;

#[derive(Debug, Clone)]
pub struct Channel {
    id: Uuid,
    workspace_id: Uuid,
    code: String,
    properties: Json,
}

impl Channel {
    pub fn new(conn: &PgConnection, ws: &Workspace, _code: &String) -> Result<Channel> {
        let _propert = json!("{}");

        let channel = Channel {
            id: Uuid::new_v4(),
            workspace_id: ws.id(),
            code: _code.to_owned(),
            properties: _propert,
        };

        Ok(channel)
    }
}
