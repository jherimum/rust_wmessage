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
    pub fn new(clear_password: &str) -> Result<Password, AppError> {
        Encrypter::new()
            .encrypt(&clear_password)
            .map(|_hash| Password {
                id: Uuid::new_v4(),
                hash: _hash,
            })
    }

    pub fn save(self, conn: &mut PgConnection) -> Result<Password, AppError> {
        match insert_into(passwords).values(&self).execute(conn) {
            Ok(1) => Ok(self),
            Ok(_) => Err(AppError::model_error(
                super::ModelErrorKind::EntityNotFound {
                    message: "password not inserted".to_string(),
                },
            )),
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

    pub fn authenticate(&self, clear_password: &str) -> Result<bool, AppError> {
        Encrypter::new().verify(clear_password, &self.hash)
    }
}
