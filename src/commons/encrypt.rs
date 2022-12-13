use super::error::AppError;
use mockall::automock;

#[automock]
pub trait Encrypter {
    fn encrypt(&self, plain_input: &str) -> Result<String, AppError>;
    fn verify(&self, plain_input: &str, hash: &str) -> Result<bool, AppError>;
}

pub mod argon {
    use argon2::{
        password_hash::SaltString, Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
    };
    use rand::rngs::OsRng;

    use crate::commons::error::AppError;

    use super::Encrypter;

    pub struct Argon<'a>(Argon2<'a>);

    impl Encrypter for Argon<'_> {
        fn encrypt(&self, clear_password: &str) -> Result<String, AppError> {
            let salt = SaltString::generate(&mut OsRng);
            self.0
                .hash_password(clear_password.as_bytes(), &salt)
                .map(|ph| ph.to_string())
                .map_err(AppError::from)
        }

        fn verify(&self, clear_password: &str, hash: &str) -> Result<bool, AppError> {
            let password_hash = PasswordHash::new(hash).map_err(AppError::from)?;
            match self
                .0
                .verify_password(clear_password.as_bytes(), &password_hash)
            {
                Ok(_) => Ok(true),
                Err(_) => Ok(false),
            }
        }
    }

    impl Argon<'_> {
        pub fn new() -> Self {
            Argon(Argon2::default())
        }
    }

    impl Default for Argon<'_> {
        fn default() -> Self {
            Argon::new()
        }
    }
}
#[cfg(test)]
mod tests {
    use crate::commons::encrypt::{argon::Argon, Encrypter};

    #[test]
    fn test_encrypter_verify() {
        const PASSWORD_1: &str = "password1";
        const PASSWORD_2: &str = "password2";

        let e = Argon::new();
        let hash = e.encrypt(PASSWORD_1).unwrap();

        assert!(e.verify(PASSWORD_1, &hash).unwrap());
        assert!(!e.verify(PASSWORD_2, &hash).unwrap());
    }
}
