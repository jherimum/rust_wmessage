use std::future::{ready, Ready};

use actix_web::{web::Data, FromRequest};
use diesel::Connection;

use std::ops::Deref;

use serde::Deserialize;

use crate::{
    config::DbPool,
    error::AppError,
    models::{password::Password, user::User, workspace::Workspace},
};

use super::crypto::PasswordCrypto;

#[derive(Deserialize, Debug)]
pub struct RegistrationForm {
    workspace_code: String,
    user_email: String,
    user_password: String,
}

pub struct RegistrationService {
    pool: DbPool,
    crypto: PasswordCrypto,
}

impl RegistrationService {
    pub fn register(self, form: RegistrationForm) -> Result<(), AppError> {
        let conn = &mut self.pool.get().map_err(|e| AppError::PoolError(e))?;

        conn.transaction::<_, AppError, _>(|conn| {
            let hash = self.crypto.encrypt(&form.user_password)?;

            let ws = Workspace::create(conn, &form.workspace_code)?;
            let user = User::create_owner(conn, &ws, &form.user_email)?;
            let _password = Password::create(conn, &user, &hash);
            Ok(())
        })
    }

    pub fn new(pool: DbPool, crypto: PasswordCrypto) -> Self {
        Self {
            pool: pool,
            crypto: crypto,
        }
    }
}

impl FromRequest for RegistrationService {
    type Error = AppError;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(
        req: &actix_web::HttpRequest,
        payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        let poll = Data::<DbPool>::from_request(req, payload).into_inner();
        let crypto = Data::<PasswordCrypto>::from_request(req, payload).into_inner();

        match poll {
            Ok(p) => match crypto {
                Ok(c) => ready(Ok(RegistrationService::new(
                    p.deref().deref().clone(),
                    c.deref().deref().clone(),
                ))),
                _ => ready(Err(AppError::Unknow)),
            },
            _ => ready(Err(AppError::Unknow)),
        }
    }
}
