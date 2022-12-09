use super::password::Password;
use super::workspace::Workspace;
use crate::commons::error::AppError;
use crate::schema::{self, users};
use diesel::prelude::*;
use diesel::{insert_into, PgConnection};
use schema::users::dsl::*;
use uuid::Uuid;

#[derive(Insertable, Queryable, Identifiable, Debug, Clone, PartialEq, Eq)]
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
        users
            .filter(workspace_id.eq(&ws.id()).and(owner.eq(true)))
            .first::<User>(conn)
            .optional()
            .map_err(AppError::from)
    }

    pub fn id(&self) -> Uuid {
        self.id
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
        match insert_into(users).values(&self).execute(conn) {
            Ok(_) => Ok(self),
            Err(_) => Err(AppError::database_error("password not inserted")),
        }
    }

    pub fn new(
        _conn: &mut PgConnection,
        ws: &Workspace,
        _email: &str,
        password: &Password,
        _owner: bool,
    ) -> User {
        User {
            id: Uuid::new_v4(),
            email: _email.to_string(),
            workspace_id: ws.id(),
            password_id: password.id(),
            owner: _owner,
        }
    }
}
