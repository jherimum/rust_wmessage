use diesel::insert_into;
use diesel::prelude::*;
use diesel::PgConnection;
use uuid::Uuid;

use crate::error::AppError;
use crate::schema::{self, workspaces};

use schema::workspaces::dsl::*;

#[derive(Insertable, Queryable)]
pub struct Workspace {
    id: Uuid,
    code: String,
}

impl Workspace {
    pub fn id(&self) -> Uuid {
        self.id
    }

    fn exists_code(conn: &mut PgConnection, _code: &String) -> Result<bool, AppError> {
        workspaces::table
            .filter(code.eq(_code))
            .count()
            .get_result::<i64>(conn)
            .map_err(|e| AppError::DatabaseError(e))
            .map(|count| count > 0)
    }

    pub fn create(conn: &mut PgConnection, _code: &String) -> Result<Workspace, AppError> {
        if Workspace::exists_code(conn, _code)? {
            return Err(AppError::WorkspaceWithCodeAlreadyExistsError);
        }

        let ws = Workspace {
            id: Uuid::new_v4(),
            code: _code.to_owned(),
        };

        let _x = insert_into(schema::workspaces::dsl::workspaces)
            .values(&ws)
            .execute(conn);

        Ok(ws)
    }
}