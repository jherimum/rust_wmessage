use super::password::Password;
use super::workspace::Workspace;
use crate::commons::error::AppError;
use crate::commons::error::IntoAppError;
use crate::commons::Result;
use crate::schema::users;
use derive_getters::Getters;
use diesel::insert_into;
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
