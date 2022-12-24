use crate::commons::encrypt::argon::Argon;
use crate::commons::id::id::new_id;
use crate::commons::types::Conn;
use crate::commons::types::Result;
use crate::models::password::Password;
use crate::models::user::User;
use crate::models::workspace::Workspace;
use crate::resources::registrations::RegistrationForm;
use diesel::Connection;

pub struct RegistrationService;

impl RegistrationService {
    pub fn register(conn: &mut Conn, form: RegistrationForm) -> Result<User> {
        conn.transaction(|conn| {
            let ws = Self::create_ws(conn, form.workspace_code)?;
            let password = Self::create_pass(conn, form.user_password)?;
            let user = Self::create_user(conn, ws, password, form.user_email)?;
            Ok(user)
        })
    }

    fn create_user(conn: &mut Conn, ws: Workspace, pass: Password, email: String) -> Result<User> {
        let user = User::new(conn, new_id(), ws, &email, pass, true);
        User::save(conn, user)
    }

    fn create_ws(conn: &mut Conn, code: String) -> Result<Workspace> {
        let ws = Workspace::new(new_id(), code);
        Workspace::save(conn, ws)
    }

    fn create_pass(conn: &mut Conn, pass: String) -> Result<Password> {
        let password = Password::new(new_id(), &pass, &Argon::new())?;
        Password::save(conn, password)
    }
}
