use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use diesel::prelude::*;
use diesel::{insert_into, PgConnection};

use rand::rngs::OsRng;
use uuid::Uuid;

use crate::schema::passwords::dsl::*;

use crate::schema::passwords;
use anyhow::{anyhow, bail, Context};

pub struct Encrypter<'a>(Argon2<'a>);

impl Encrypter<'_> {
    fn argon(&self) -> Argon2 {
        self.0.clone()
    }

    pub fn new() -> Self {
        Encrypter(Argon2::default())
    }

    pub fn encrypt(&self, clear_password: &str) -> anyhow::Result<String> {
        let _salt = SaltString::generate(&mut OsRng);
        self.argon()
            .hash_password(clear_password.as_bytes(), &_salt)
            .map(|ph| ph.to_string())
            .map_err(|e| anyhow!(e))
            .context("error while hashing password")
    }

    pub fn verify(&self, clear_password: &str, _hash: &str) -> anyhow::Result<bool> {
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

#[derive(Insertable, Queryable, Identifiable, Debug, Clone, PartialEq)]
#[diesel(table_name = passwords)]
pub struct Password {
    id: Uuid,
    hash: String,
}

impl Password {
    pub fn new(clear_password: &str) -> anyhow::Result<Password> {
        Encrypter::new()
            .encrypt(&clear_password)
            .map(|_hash| Password {
                id: Uuid::new_v4(),
                hash: _hash,
            })
            .context("failed to encrypt password")
    }

    pub fn save(self, conn: &mut PgConnection) -> anyhow::Result<Password> {
        match insert_into(passwords).values(&self).execute(conn) {
            Ok(_) => Ok(self),
            Err(e) => bail!(e),
        }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn find(conn: &mut PgConnection, _id: &Uuid) -> anyhow::Result<Option<Password>> {
        passwords
            .filter(id.eq(_id))
            .first::<Password>(conn)
            .optional()
            .context("failed to retrieve password")
    }

    pub fn authenticate(&self, clear_password: &str) -> anyhow::Result<bool> {
        Encrypter::new().verify(clear_password, &self.hash)
    }
}

#[cfg(test)]
mod tests {

    use super::{Encrypter, Password};

    #[test]
    fn test_encrypter_verify() {
        let e = Encrypter::new();
        let hash = e.encrypt("password").unwrap();
        assert!(e.verify("password", &hash).unwrap());
        assert!(!e.verify("password1", &hash).unwrap());
    }

    #[test]
    fn test_password_authetication() {
        let p = Password::new("password").unwrap();

        assert!(p.authenticate("password").unwrap());
        assert!(!p.authenticate("password1").unwrap());
    }
}
