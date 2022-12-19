use crate::app::routes::registration_routes::RegistrationForm;
use crate::commons::encrypt::argon::Argon;
use crate::commons::Result;
use crate::models::password::Password;
use crate::models::user::User;
use crate::models::workspace::Workspace;
use crate::repository::password_repo::Passwords;
use crate::repository::user_repo::Users;
use crate::repository::workspace_repo::Workspaces;
use diesel::Connection;
use diesel::PgConnection;

pub struct RegistrationService;

impl RegistrationService {
    pub fn register(conn: &mut PgConnection, form: RegistrationForm) -> Result<User> {
        conn.transaction(|conn| {
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
