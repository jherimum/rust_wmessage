use crate::commons::encrypt::Encrypter;
use crate::commons::error::AppError;
use crate::commons::uuid::new_uuid;
use crate::schema::passwords;
use derive_getters::Getters;
use diesel::prelude::*;
use diesel::{insert_into, PgConnection};
use uuid::Uuid;

#[derive(Insertable, Queryable, Identifiable, Debug, Clone, PartialEq, Eq, Getters)]
#[diesel(table_name = passwords)]
pub struct Password {
    pub id: Uuid,
    pub hash: String,
}

impl Password {
    pub fn new_with_id(
        id: Uuid,
        plain_password: &str,
        encrypter: &dyn Encrypter,
    ) -> Result<Password, AppError> {
        encrypter.encrypt(plain_password).map(|_hash| Password {
            id: id,
            hash: _hash,
        })
    }

    pub fn new(plain_password: &str, encrypter: &dyn Encrypter) -> Result<Password, AppError> {
        Self::new_with_id(new_uuid(), plain_password, encrypter)
    }

    pub fn save(self, conn: &mut PgConnection) -> Result<Password, AppError> {
        match insert_into(passwords::table).values(&self).execute(conn) {
            Ok(1) => Ok(self),
            Ok(_) => Err(AppError::database_error("password not inserted")),
            Err(err) => Err(AppError::from(err)),
        }
    }

    pub fn find(conn: &mut PgConnection, id: Uuid) -> Result<Option<Password>, AppError> {
        passwords::table
            .filter(passwords::id.eq(id))
            .first::<Password>(conn)
            .optional()
            .map_err(AppError::from)
    }

    pub fn authenticate(
        &self,
        plain_password: &str,
        encrypter: &dyn Encrypter,
    ) -> Result<bool, AppError> {
        encrypter.verify(plain_password, &self.hash)
    }
}

#[cfg(test)]
mod test {
    use crate::commons::{encrypt::MockEncrypter, uuid::new_uuid};

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
        let id = new_uuid();
        let pass = Password::new_with_id(id, "password", &mock_encrypt()).unwrap();
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
        let pass = Password::new("password", &encrypt).unwrap();
        assert!(pass.authenticate("password", &encrypt).unwrap());
        assert!(!pass.authenticate("password1", &encrypt).unwrap());
    }
}
