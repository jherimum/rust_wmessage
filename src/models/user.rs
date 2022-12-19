use super::password::Password;
use super::workspace::Workspace;
use crate::schema::users;
use derive_getters::Getters;
use diesel::prelude::*;
use diesel::PgConnection;
use uuid::Uuid;

#[derive(Insertable, Queryable, Identifiable, Debug, Clone, PartialEq, Eq, Getters)]
#[diesel(table_name = users)]
pub struct User {
    id: Uuid,
    email: String,
    workspace_id: Uuid,
    owner: bool,
    password_id: Uuid,
}

impl User {
    pub fn new(
        _conn: &mut PgConnection,
        id: Uuid,
        ws: Workspace,
        email: &str,
        password: Password,
        owner: bool,
    ) -> User {
        User {
            id: id,
            email: email.to_string(),
            workspace_id: ws.id().clone(),
            password_id: password.id().clone(),
            owner: owner,
        }
    }
}
