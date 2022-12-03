use diesel::prelude::*;
use diesel::{insert_into, PgConnection};
use uuid::Uuid;

use schema::passwords::dsl::*;

use super::user::User;
use crate::schema::{self, passwords};
use anyhow::{Error, Result};

#[derive(Insertable, Debug)]
pub struct Password {
    user_id: Uuid,
    hash: String,
}

impl Password {
    pub fn create(conn: &mut PgConnection, user: &User, _hash: &String) -> Result<Password> {
        let p = Password {
            user_id: user.id(),
            hash: _hash.to_owned(),
        };

        match insert_into(passwords).values(&p).execute(conn) {
            Ok(_) => Ok(p),
            Err(e) => Err(Error::new(e)),
        }
    }
}
