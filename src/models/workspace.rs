use super::user::User;
use super::ModelErrorKind;
use crate::commons::error::AppError;
use crate::commons::error::IntoAppError;
use crate::commons::uuid::new_uuid;
use crate::schema::workspaces;
use derive_getters::Getters;
use diesel::prelude::*;
use diesel::OptionalExtension;
use diesel::{insert_into, PgConnection};
use uuid::Uuid;

#[derive(Insertable, Identifiable, Debug, Clone, PartialEq, Queryable, Eq, Getters)]
#[diesel(table_name = workspaces)]
pub struct Workspace {
    id: Uuid,
    code: String,
}

impl Workspace {
    pub fn new(code: &str) -> Self {
        Workspace {
            id: new_uuid(),
            code: code.to_string(),
        }
    }

    pub fn owner(&self, conn: &mut PgConnection) -> Result<User, AppError> {
        match User::ws_owner(conn, self)? {
            Some(u) => Ok(u),
            None => Err(AppError::model_error(super::ModelErrorKind::EntityNotFound)),
        }
    }

    pub fn find(conn: &mut PgConnection, id: &Uuid) -> Result<Option<Workspace>, AppError> {
        workspaces::table
            .filter(workspaces::id.eq(id))
            .first::<Workspace>(conn)
            .optional()
            .into_app_error()
    }

    pub fn exists_code(conn: &mut PgConnection, code: &str) -> Result<bool, AppError> {
        workspaces::table
            .filter(workspaces::code.eq(code))
            .count()
            .get_result::<i64>(conn)
            .map(|count| count > 0)
            .into_app_error()
    }

    pub fn save(self, conn: &mut PgConnection) -> Result<Workspace, AppError> {
        if Workspace::exists_code(conn, &self.code)? {
            return Err(AppError::model_error(
                ModelErrorKind::WorkspaceCodeAlreadyExists { code: self.code },
            ));
        }

        match insert_into(workspaces::table)
            .values(&self)
            .execute(conn)
            .map_err(AppError::from)?
        {
            1 => Ok(self),
            _ => Err(AppError::database_error("Workspace not inserted")),
        }
    }
}
