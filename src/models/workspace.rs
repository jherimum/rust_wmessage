use crate::{
    commons::{
        error::{AppError, IntoAppError},
        Id, Result,
    },
    schema::workspaces,
};
use derive_getters::Getters;
use diesel::{insert_into, prelude::*};

use super::{Code, ModelErrorKind};

#[derive(Insertable, Identifiable, Debug, Clone, PartialEq, Queryable, Eq, Getters)]
#[diesel(table_name = workspaces)]
pub struct Workspace {
    id: Id,
    code: Code,
}

impl Workspace {
    pub fn new(id: Id, code: Code) -> Self {
        Workspace { id: id, code: code }
    }

    pub fn find(conn: &mut PgConnection, id: &Id) -> Result<Option<Workspace>> {
        workspaces::table
            .filter(workspaces::id.eq(id))
            .first::<Workspace>(conn)
            .optional()
            .into_app_error()
    }

    pub fn exists_code(conn: &mut PgConnection, code: &str) -> Result<bool> {
        workspaces::table
            .filter(workspaces::code.eq(code))
            .count()
            .get_result::<i64>(conn)
            .map(|count| count > 0)
            .into_app_error()
    }

    pub fn save(conn: &mut PgConnection, ws: Workspace) -> Result<Workspace> {
        if Self::exists_code(conn, &ws.code())? {
            return Err(AppError::model_error(
                ModelErrorKind::WorkspaceCodeAlreadyExists {
                    code: ws.code().to_owned(),
                },
            ));
        }

        match insert_into(workspaces::table)
            .values(&ws)
            .execute(conn)
            .map_err(AppError::from)?
        {
            1 => Ok(ws),
            _ => Err(AppError::database_error("Workspace not inserted")),
        }
    }
}
