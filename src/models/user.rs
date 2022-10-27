use diesel::{insert_into, PgConnection};
use uuid::Uuid;

use diesel::prelude::*;
use schema::users::dsl::*;

use crate::error::AppError;
use crate::models::workspace::Workspace;
use crate::schema::{self, users};

#[derive(Insertable, Queryable, Debug)]
pub struct User {
    id: Uuid,
    email: String,
    workspace_id: Uuid,
    owner: bool,
}

impl User {
    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn create_owner(
        conn: &mut PgConnection,
        ws: &Workspace,
        _email: &String,
    ) -> Result<User, AppError> {
        let user = User {
            id: Uuid::new_v4(),
            email: _email.clone(),
            workspace_id: ws.id(),
            owner: true,
        };

        insert_into(users)
            .values(&user)
            .execute(conn)
            .map_err(|e| AppError::DatabaseError(e))?;

        Ok(user)
    }
}
