use super::user::User;
use crate::commons::error::AppError;
use crate::commons::uuid::new_uuid;
use crate::schema::{self, workspaces};
use diesel::prelude::*;
use diesel::OptionalExtension;
use diesel::{insert_into, PgConnection};
use schema::workspaces::dsl::*;
use uuid::Uuid;

#[derive(Insertable, Identifiable, Debug, Clone, PartialEq, Queryable, Eq)]
#[diesel(table_name = workspaces)]
pub struct Workspace {
    id: Uuid,
    code: String,
}

impl Workspace {
    pub fn new(ws_code: &str) -> Self {
        Workspace {
            id: new_uuid(),
            code: ws_code.to_string(),
        }
    }

    pub fn owner(&self, conn: &mut PgConnection) -> Result<User, AppError> {
        match User::ws_owner(conn, self)? {
            Some(u) => Ok(u),
            None => Err(AppError::model_error(
                super::ModelErrorKind::EntityNotFound {
                    message: "owner not found".to_string(),
                },
            )),
        }
    }

    pub fn find(conn: &mut PgConnection, ws_id: &Uuid) -> Result<Option<Workspace>, AppError> {
        workspaces
            .filter(id.eq(ws_id))
            .first::<Workspace>(conn)
            .optional()
            .map_err(AppError::from)
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn exists_code(conn: &mut PgConnection, _code: &str) -> Result<bool, AppError> {
        workspaces::table
            .filter(code.eq(_code))
            .count()
            .get_result::<i64>(conn)
            .map(|count| count > 0)
            .map_err(AppError::from)
    }

    pub fn save(self, conn: &mut PgConnection) -> Result<Workspace, AppError> {
        if Workspace::exists_code(conn, &self.code)? {
            return Err(AppError::model_error(
                crate::models::ModelErrorKind::WorkspaceCodeAlreadyExists {
                    code: self.code.clone(),
                },
            ));
        }

        match insert_into(schema::workspaces::dsl::workspaces)
            .values(&self)
            .execute(conn)
            .map_err(AppError::from)?
        {
            1 => Ok(self),
            _ => Err(AppError::database_error("Workspace not inserted")),
        }
    }
}
