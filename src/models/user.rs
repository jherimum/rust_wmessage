use diesel::{insert_into, PgConnection};
use uuid::Uuid;

use diesel::prelude::*;
use schema::users::dsl::*;

use crate::models::workspace::Workspace;
use crate::schema::{self, users};

use anyhow::{bail, Result};

use super::password::Password;

#[derive(Insertable, Queryable, Identifiable, Debug, Clone, PartialEq)]
pub struct User {
    id: Uuid,
    email: String,
    workspace_id: Uuid,
    password_id: Uuid,
    owner: bool,
}

impl User {
    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn password(&self, conn: &mut PgConnection) -> Result<Password> {
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
        password: &Password,
    ) -> Result<User> {
        let user = User {
            id: Uuid::new_v4(),
            email: _email.clone(),
            workspace_id: ws.id(),
            password_id: password.id(),
            owner: true,
        };

        match insert_into(users).values(&user).execute(conn) {
            Ok(_) => Ok(user),
            Err(e) => bail!(e),
        }
    }
}
