use crate::commons::error::AppError;
use crate::commons::error::IntoAppError;
use crate::commons::Result;
use crate::models::password::Password;
use crate::schema::passwords;
use diesel::prelude::*;
use diesel::{insert_into, PgConnection};
use uuid::Uuid;

pub struct Passwords;

impl Passwords {
    pub fn save(conn: &mut PgConnection, password: Password) -> Result<Password> {
        match insert_into(passwords::table)
            .values(&password)
            .execute(conn)
        {
            Ok(1) => Ok(password),
            Ok(_) => Err(AppError::database_error("password not inserted")),
            Err(err) => Err(AppError::from(err)),
        }
    }

    pub fn find(conn: &mut PgConnection, id: Uuid) -> Result<Option<Password>> {
        passwords::table
            .filter(passwords::id.eq(id))
            .first::<Password>(conn)
            .optional()
            .into_app_error()
    }
}
