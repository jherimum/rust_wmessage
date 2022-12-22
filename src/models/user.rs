use super::password::Password;
use super::workspace::Workspace;
use crate::commons::error::AppError;
use crate::commons::error::IntoAppError;
use crate::commons::types::Id;
use crate::commons::types::Result;
use crate::schema::users;
use derive_getters::Getters;
use diesel::insert_into;
use diesel::prelude::*;
use diesel::PgConnection;

#[derive(Insertable, Queryable, Identifiable, Debug, Clone, PartialEq, Eq, Getters)]
#[diesel(table_name = users)]
pub struct User {
    id: Id,
    email: String,
    workspace_id: Id,
    owner: bool,
    password_id: Id,
}

impl User {
    pub fn new(
        _conn: &mut PgConnection,
        id: Id,
        ws: Workspace,
        email: &str,
        password: Password,
        owner: bool,
    ) -> User {
        User {
            id,
            email: email.to_string(),
            workspace_id: *ws.id(),
            password_id: *password.id(),
            owner,
        }
    }

    pub fn save(conn: &mut PgConnection, user: User) -> Result<User> {
        match insert_into(users::table).values(&user).execute(conn) {
            Ok(_) => Ok(user),
            Err(_) => Err(AppError::database_error("password not inserted")),
        }
    }

    pub fn ws_owner(conn: &mut PgConnection, ws: Workspace) -> Result<Option<User>> {
        users::table
            .filter(users::workspace_id.eq(&ws.id()).and(users::owner.eq(true)))
            .first::<User>(conn)
            .optional()
            .into_app_error()
    }
}
