use argon2::{
    password_hash::{rand_core::OsRng, PasswordVerifier, SaltString},
    Argon2, PasswordHasher,
};

use crate::error::AppError;

#[derive(Debug, Clone)]
pub struct PasswordCrypto {}

impl PasswordCrypto {
    pub fn new() -> PasswordCrypto {
        PasswordCrypto {}
    }

    pub fn encrypt(&self, pwd: &String) -> Result<String, AppError> {
        let salt = SaltString::generate(OsRng);
        let argon = argon2::Argon2::default();

        match argon.hash_password(&pwd.as_bytes(), &salt) {
            Ok(hash) => Ok(hash.to_string()),
            _ => Err(AppError::Unknow),
        }
    }
}
