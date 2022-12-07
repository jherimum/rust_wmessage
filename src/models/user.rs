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
            .context("context")
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

    pub fn create_owner(
        conn: &mut PgConnection,
        ws: &Workspace,
        _email: &String,
        clear_password: &str,
    ) -> anyhow::Result<User> {
        let user = User {
            id: Uuid::new_v4(),
            email: _email.clone(),
            workspace_id: ws.id(),
            password_id: Password::create(conn, clear_password)?.id(),
            owner: true,
        };

        match insert_into(users).values(&user).execute(conn) {
            Ok(_) => Ok(user),
            Err(e) => bail!(e),
        }
    }
}
