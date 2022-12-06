use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use diesel::prelude::*;
use diesel::{insert_into, PgConnection};

use rand::rngs::OsRng;
use uuid::Uuid;

use crate::schema::passwords::dsl::*;

use crate::schema::passwords;
use anyhow::{anyhow, Context, Error, Result};

#[derive(Insertable, Debug)]
#[diesel(table_name = passwords)]
pub struct Password {
    id: Uuid,
    hash: String,
}

struct Encrypter<'a>(Argon2<'a>);

impl Encrypter<'_> {
    fn argon(&self) -> Argon2 {
        self.0.clone()
    }

    fn new() -> Self {
        Encrypter(Argon2::default())
    }

    fn encrypt(&self, clear_password: &str) -> Result<String> {
        let _salt = SaltString::generate(&mut OsRng);
        self.argon()
            .hash_password(clear_password.as_bytes(), &_salt)
            .map(|ph| ph.to_string())
            .map_err(|e| anyhow!(e))
            .context("error while hashing password")
    }

    fn verify(&self, clear_password: &str, _hash: &str) -> Result<bool> {
        let password_hash = PasswordHash::new(_hash)
            .map_err(|e| anyhow!(e))
            .context("error while creating PasswordHash")?;

        match self
            .argon()
            .verify_password(&clear_password.as_bytes(), &password_hash)
        {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }
}

impl Password {
    pub fn authenticate(&self, clear_password: &str) -> Result<bool> {
        Encrypter::new().verify(clear_password, &self.hash)
    }

    pub fn create(conn: &mut PgConnection, clear_password: &str) -> Result<Password> {
        let _hash = Encrypter::new().encrypt(clear_password)?;

        let p = Password {
            id: Uuid::new_v4(),
            hash: _hash,
        };

        match insert_into(passwords).values(&p).execute(conn) {
            Ok(_) => Ok(p),
            Err(e) => Err(Error::new(e)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Password;

    #[test]
    fn t() {}
}
