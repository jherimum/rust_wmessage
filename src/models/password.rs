use crate::commons::error::IntoAppError;
use crate::commons::{encrypt::Encrypter, error::AppError};
use crate::commons::{Id, Result};
use crate::schema::passwords;
use derive_getters::Getters;
use diesel::{insert_into, prelude::*};
use uuid::Uuid;

#[derive(Insertable, Queryable, Identifiable, Debug, Clone, PartialEq, Eq, Getters)]
#[diesel(table_name = passwords)]
pub struct Password {
    pub id: Id,
    pub hash: String,
}

impl Password {
    pub fn new(id: Id, plain_password: &str, encrypter: &dyn Encrypter) -> Result<Password> {
        encrypter.encrypt(plain_password).map(|_hash| Password {
            id: id,
            hash: _hash,
        })
    }

    pub fn authenticate(&self, plain_password: &str, encrypter: &dyn Encrypter) -> Result<bool> {
        encrypter.verify(plain_password, &self.hash)
    }

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

#[cfg(test)]
mod test {
    use crate::commons::{encrypt::MockEncrypter, id::Id::new_id};

    use super::Password;

    fn mock_encrypt() -> MockEncrypter {
        let mut mock = MockEncrypter::new();
        mock.expect_encrypt().returning(|pass| Ok(pass.to_string()));
        mock.expect_verify()
            .returning(|pass, hash| Ok(pass.eq(hash)));
        mock
    }

    #[test]
    fn test_new_password() {
        let id = new_id();
        let pass = Password::new(id, "password", &mock_encrypt()).unwrap();
        assert_eq!(
            pass,
            Password {
                id: id,
                hash: "password".to_string()
            }
        );
    }

    #[test]
    fn test_authenticate() {
        let encrypt = mock_encrypt();
        let pass = Password::new(new_id(), "password", &encrypt).unwrap();
        assert!(pass.authenticate("password", &encrypt).unwrap());
        assert!(!pass.authenticate("password1", &encrypt).unwrap());
    }
}
