use diesel::prelude::*;
use diesel::{insert_into, PgConnection};
use uuid::Uuid;

use crate::models::Error;
use crate::schema::{self, workspaces};

use anyhow::Result;
use anyhow::{bail, Context};

use diesel::OptionalExtension;
use schema::workspaces::dsl::*;

use super::user::User;

#[derive(Insertable, Identifiable, Debug, Clone, PartialEq, Queryable)]
#[diesel(table_name = workspaces)]
pub struct Workspace {
    id: Uuid,
    code: String,
}

impl Workspace {
    pub fn new(ws_id: Uuid, ws_code: &str) -> Self {
        Workspace {
            id: ws_id,
            code: ws_code.to_string(),
        }
    }

    pub fn owner(&self, conn: &mut PgConnection) -> Result<User> {
        match User::ws_owner(conn, &self)? {
            Some(u) => Ok(u),
            None => bail!("owner not found"),
        }
    }

    pub fn find(conn: &mut PgConnection, ws_id: &Uuid) -> anyhow::Result<Option<Workspace>> {
        workspaces
            .filter(id.eq(ws_id))
            .first::<Workspace>(conn)
            .optional()
            .context("Database error")
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn exists_code(conn: &mut PgConnection, _code: &str) -> Result<bool> {
        workspaces::table
            .filter(code.eq(_code))
            .count()
            .get_result::<i64>(conn)
            .map(|count| count > 0)
            .context("Database error")
    }

    pub fn create(conn: &mut PgConnection, _code: &str) -> Result<Workspace> {
        if Workspace::exists_code(conn, _code)? {
            bail!(Error::WS001 {
                code: _code.to_string(),
            });
        }

        let ws = Self::new(Uuid::new_v4(), _code);

        let rows_inserted: usize = insert_into(schema::workspaces::dsl::workspaces)
            .values(&ws)
            .execute(conn)
            .context("Database error")?;

        match rows_inserted {
            1 => Ok(ws),
            _ => bail!("The workspace could not be inserted"),
        }
    }
}
