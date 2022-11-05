use diesel::{insert_into, PgConnection};
use uuid::Uuid;

use crate::schema::{self, workspaces};

use diesel::prelude::*;
use diesel::OptionalExtension;
use schema::workspaces::dsl::*;

use anyhow::{bail, Context, Result};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("data store disconnected")]
    WS_001 { _code: String },
}

#[derive(Insertable, Queryable, Identifiable, Debug, Clone)]
#[diesel(table_name = workspaces)]
pub struct Workspace {
    id: Uuid,
    code: String,
}

impl Workspace {
    pub fn find(conn: &mut PgConnection, ws_id: &Uuid) -> Result<Option<Workspace>> {
        workspaces
            .filter(id.eq(ws_id))
            .first::<Workspace>(conn)
            .optional()
            .context("database error")
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    fn exists_code(conn: &mut PgConnection, _code: &String) -> Result<bool> {
        workspaces::table
            .filter(code.eq(_code))
            .count()
            .get_result::<i64>(conn)
            .map(|count| count > 0)
            .context("database error")
    }

    pub fn create(conn: &mut PgConnection, _code: &String) -> Result<Workspace> {
        if Workspace::exists_code(conn, _code)? {
            let error = anyhow::Error::new(Error::WS_001 {
                _code: _code.to_owned(),
            });
            return Err(error);
        }

        let ws = Workspace {
            id: Uuid::new_v4(),
            code: _code.to_owned(),
        };

        insert_into(schema::workspaces::dsl::workspaces)
            .values(&ws)
            .execute(conn)
            .context("database error")?;

        Ok(ws)
    }
}
