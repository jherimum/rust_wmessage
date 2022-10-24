use diesel::prelude::*;
use diesel::{insert_into, PgConnection};
use uuid::Uuid;

use schema::passwords::dsl::*;

use crate::error::AppError;
use crate::schema::{self, passwords};

use super::user::User;

#[derive(Insertable, Queryable, Debug)]
pub struct Password {
    user_id: Uuid,
    hash: String,
}

impl Password {
    pub fn create(
        conn: &mut PgConnection,
        user: &User,
        _hash: &String,
    ) -> Result<Password, AppError> {
        let p = Password {
            user_id: user.id(),
            hash: _hash.to_owned(),
        };

        insert_into(passwords)
            .values(&p)
            .execute(conn)
            .map_err(|e| AppError::DatabaseError(e))?;

        Ok(p)
    }
}