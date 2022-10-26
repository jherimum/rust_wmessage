use diesel::PgConnection;
use uuid::Uuid;

use crate::error::AppError;

use super::workspace::Workspace;

#[derive(Debug, Clone)]
pub struct Channel {
    id: Uuid,
    workspace_id: Uuid,
    code: String,
}

impl Channel {
    pub fn new(conn: &PgConnection, ws: &Workspace, _code: &String) -> Result<Channel, AppError> {
        let channel = Channel {
            id: Uuid::new_v4(),
            workspace_id: ws.id(),
            code: _code.to_owned(),
        };

        Ok(channel)
    }
}
