use crate::commons::encrypt::Encrypter;
use crate::commons::error::AppError;
use crate::schema::passwords::dsl::*;
use diesel::prelude::*;
use diesel::{insert_into, PgConnection};
use uuid::Uuid;

use crate::schema::passwords;

#[derive(Insertable, Queryable, Identifiable, Debug, Clone, PartialEq)]
#[diesel(table_name = passwords)]
pub struct Password {
    id: Uuid,
    hash: String,
}

impl Password {
    pub fn new(plain_password: &str, encrypter: &dyn Encrypter) -> Result<Password, AppError> {
        encrypter.encrypt(&plain_password).map(|_hash| Password {
            id: Uuid::new_v4(),
            hash: _hash,
        })
    }

    pub fn save(self, conn: &mut PgConnection) -> Result<Password, AppError> {
        match insert_into(passwords).values(&self).execute(conn) {
            Ok(1) => Ok(self),
            Ok(_) => Err(AppError::database_error("password not inserted")),
            Err(err) => Err(AppError::from(err)),
        }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn find(conn: &mut PgConnection, _id: &Uuid) -> Result<Option<Password>, AppError> {
        passwords
            .filter(id.eq(_id))
            .first::<Password>(conn)
            .optional()
            .map_err(|err| AppError::from(err))
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
    use crate::commons::encrypt::MockEncrypter;

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
        let pass = Password::new("password", &mock_encrypt()).unwrap();
        assert_eq!(pass.hash, "password");
    }

    #[test]
    fn test_authenticate() {
        let encrypt = mock_encrypt();
        let pass = Password::new("password", &encrypt).unwrap();
        assert!(pass.authenticate("password", &encrypt).unwrap());
        assert!(!pass.authenticate("password1", &encrypt).unwrap());
    }
}
