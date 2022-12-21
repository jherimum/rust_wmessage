use crate::app::routes::registration_routes::RegistrationForm;
use crate::commons::encrypt::argon::Argon;
use crate::commons::mock_uuid::new_uuid;
use crate::commons::Result;
use crate::models::password::Password;
use crate::models::user::User;
use crate::models::workspace::Workspace;
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
        let user = User::new(conn, new_uuid(), ws, &email, pass, true);
        User::save(conn, user)
    }

    fn create_ws(conn: &mut PgConnection, code: String) -> Result<Workspace> {
        let ws = Workspace::new(new_uuid(), &code);
        Workspace::save(conn, ws)
    }

    fn create_pass(conn: &mut PgConnection, pass: String) -> Result<Password> {
        let password = Password::new(new_uuid(), &pass, &Argon::new())?;
        Password::save(conn, password)
    }
}
