use crate::app::routes::registration_routes::RegistrationForm;
use crate::commons::database::DbPool;
use crate::commons::encrypt::argon::Argon;
use crate::commons::error::IntoAppError;
use crate::commons::{error::AppError, Result};
use crate::models::password::Password;
use crate::models::user::User;
use crate::models::workspace::Workspace;
use crate::repository::password_repo::Passwords;
use crate::repository::user_repo::Users;
use crate::repository::workspace_repo::Workspaces;
use actix_web::{web::Data, FromRequest};
use diesel::Connection;
use diesel::{r2d2::ConnectionManager, PgConnection};
use r2d2::PooledConnection;
use std::future::{ready, Ready};

pub struct RegistrationService {
    conn: PooledConnection<ConnectionManager<PgConnection>>,
}

impl RegistrationService {
    pub fn register(&mut self, form: RegistrationForm) -> Result<User> {
        self.conn.transaction(|conn| {
            let ws = Self::create_ws(conn, form.workspace_code)?;
            let password = Self::create_pass(conn, form.user_password)?;
            let user = Self::create_user(conn, ws, password, form.user_email)?;
            Ok(user)
        })
    }

    fn create_user(
        conn: &mut PgConnection,
        ws: Workspace,
        pass: Password,
        email: String,
    ) -> Result<User> {
        let user = User::new(conn, ws, &email, pass, true);
        Users::save(conn, user)
    }

    fn create_ws(conn: &mut PgConnection, code: String) -> Result<Workspace> {
        let ws = Workspace::new(&code);
        Workspaces::save(conn, ws)
    }

    fn create_pass(conn: &mut PgConnection, pass: String) -> Result<Password> {
        let password = Password::new(&pass, &Argon::new())?;
        Passwords::save(conn, password)
    }
}

impl FromRequest for RegistrationService {
    type Error = AppError;

    type Future = Ready<Result<Self>>;

    fn from_request(req: &actix_web::HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        match req
            .app_data::<Data<DbPool>>()
            .expect("msg")
            .get()
            .into_app_error()
        {
            Ok(conn) => ready(Ok(RegistrationService { conn: conn })),
            Err(e) => ready(Err(e)),
        }
    }
}
