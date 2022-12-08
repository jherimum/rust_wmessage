use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use rand::rngs::OsRng;

use super::error::AppError;

pub struct Encrypter<'a>(Argon2<'a>);

impl Encrypter<'_> {
    fn argon(&self) -> Argon2 {
        self.0.clone()
    }

    pub fn new() -> Self {
        Encrypter(Argon2::default())
    }

    pub fn encrypt(&self, clear_password: &str) -> Result<String, AppError> {
        let _salt = SaltString::generate(&mut OsRng);
        self.argon()
            .hash_password(clear_password.as_bytes(), &_salt)
            .map(|ph| ph.to_string())
            .map_err(|e| AppError::from(e))
    }

    pub fn verify(&self, clear_password: &str, _hash: &str) -> Result<bool, AppError> {
        let password_hash = PasswordHash::new(_hash).map_err(|e| AppError::from(e))?;

        match self
            .argon()
            .verify_password(&clear_password.as_bytes(), &password_hash)
        {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::Encrypter;

    #[test]
    fn test_encrypter_verify() {
        let e = Encrypter::new();
        let hash = e.encrypt("password").unwrap();
        assert!(e.verify("password", &hash).unwrap());
        assert!(!e.verify("password1", &hash).unwrap());
    }
}
