use super::password::Password;
use super::workspace::Workspace;
use crate::schema::{self, users};
use anyhow::bail;
use anyhow::Context;
use diesel::prelude::*;
use diesel::{insert_into, PgConnection};
use schema::users::dsl::*;
use uuid::Uuid;

#[derive(Insertable, Queryable, Identifiable, Debug, Clone, PartialEq)]
#[diesel(table_name = users)]
pub struct User {
    id: Uuid,
    email: String,
    workspace_id: Uuid,
    owner: bool,
    password_id: Uuid,
}

impl User {
    pub fn ws_owner(conn: &mut PgConnection, ws: &Workspace) -> anyhow::Result<Option<User>> {
        users
            .filter(workspace_id.eq(&ws.id()).and(owner.eq(true)))
            .first::<User>(conn)
            .optional()
            .context("failed to retrieve owner")
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn password(&self, conn: &mut PgConnection) -> anyhow::Result<Password> {
        let r = Password::find(conn, &self.password_id)?;
        match r {
            Some(p) => Ok(p),
            None => bail!("password do not exists"),
        }
    }

    pub fn save(self, conn: &mut PgConnection) -> anyhow::Result<User> {
        match insert_into(users).values(&self).execute(conn) {
            Ok(_) => Ok(self),
            Err(e) => bail!(e),
        }
    }

    pub fn new(
        conn: &mut PgConnection,
        ws: &Workspace,
        _email: &String,
        password: &Password,
        _owner: bool,
    ) -> User {
        User {
            id: Uuid::new_v4(),
            email: _email.clone(),
            workspace_id: ws.id(),
            password_id: password.id(),
            owner: _owner,
        }
    }
}
