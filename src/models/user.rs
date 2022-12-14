use super::password::Password;
use super::workspace::Workspace;
use crate::commons::error::{AppError, IntoAppError};
use crate::commons::uuid::new_uuid;
use crate::schema::users;
use derive_getters::Getters;
use diesel::prelude::*;
use diesel::{insert_into, PgConnection};
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
    pub fn ws_owner(conn: &mut PgConnection, ws: &Workspace) -> Result<Option<User>, AppError> {
        users::table
            .filter(users::workspace_id.eq(&ws.id()).and(users::owner.eq(true)))
            .first::<User>(conn)
            .optional()
            .into_app_error()
    }

    pub fn password(&self, conn: &mut PgConnection) -> Result<Password, AppError> {
        let r = Password::find(conn, self.password_id)?;
        match r {
            Some(p) => Ok(p),
            None => Err(AppError::model_error(
                super::ModelErrorKind::EntityNotFound {
                    message: "Password not found".to_string(),
                },
            )),
        }
    }

    pub fn save(self, conn: &mut PgConnection) -> Result<User, AppError> {
        match insert_into(users::table).values(&self).execute(conn) {
            Ok(_) => Ok(self),
            Err(_) => Err(AppError::database_error("password not inserted")),
        }
    }

    pub fn new(
        _conn: &mut PgConnection,
        ws: &Workspace,
        email: &str,
        password: &Password,
        owner: bool,
    ) -> User {
        User {
            id: new_uuid(),
            email: email.to_string(),
            workspace_id: ws.id().clone(),
            password_id: password.id().clone(),
            owner: owner,
        }
    }
}
