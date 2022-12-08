use super::error::AppError;
use mockall::automock;

#[automock]
pub trait Encrypter {
    fn encrypt(&self, clear_password: &str) -> Result<String, AppError>;
    fn verify(&self, clear_password: &str, _hash: &str) -> Result<bool, AppError>;
}

pub mod argon {
    use argon2::{password_hash::SaltString, Argon2, PasswordHash, PasswordHasher};
    use rand::rngs::OsRng;

    use crate::commons::error::AppError;

    use super::Encrypter;

    pub struct Argon();

    impl Encrypter for Argon {
        fn encrypt(&self, clear_password: &str) -> Result<String, AppError> {
            let _salt = SaltString::generate(&mut OsRng);
            self.argon()
                .hash_password(clear_password.as_bytes(), &_salt)
                .map(|ph| ph.to_string())
                .map_err(|e| AppError::from(e))
        }

        fn verify(&self, clear_password: &str, _hash: &str) -> Result<bool, AppError> {
            let password_hash = PasswordHash::new(_hash).map_err(|e| AppError::from(e))?;
            match argon2::PasswordVerifier::verify_password(
                &self.argon(),
                &clear_password.as_bytes(),
                &password_hash,
            ) {
                Ok(_) => Ok(true),
                Err(_) => Ok(false),
            }
        }
    }

    impl Argon {
        fn argon(&self) -> Argon2<'static> {
            Argon2::default()
        }
        pub fn new() -> Self {
            Argon()
        }
    }
}
#[cfg(test)]
mod tests {
    use crate::commons::encrypt::{argon::Argon, Encrypter};

    #[test]
    fn test_encrypter_verify() {
        let e = Argon::new();

        let hash = e.encrypt("password").unwrap();
        assert!(e.verify("password", &hash).unwrap());
        assert!(!e.verify("password1", &hash).unwrap());
    }
}
