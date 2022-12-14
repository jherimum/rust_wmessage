use crate::commons::error::{AppError, IntoAppError};
use crate::commons::Result;
use crate::models::user::User;
use crate::models::workspace::Workspace;
use crate::schema::users;
use diesel::prelude::*;
use diesel::{insert_into, PgConnection};
pub struct Users;

impl Users {
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
