use crate::commons::validators::validate_password;
use crate::commons::validators::CODE_REGEX;
use crate::commons::Result;
use crate::repository::password_repo::Passwords;
use crate::repository::user_repo::Users;
use crate::repository::workspace_repo::Workspaces;
use crate::{
    commons::{encrypt::argon::Argon, error::IntoAppError},
    config::DbPool,
    models::{password::Password, user::User, workspace::Workspace},
};
use actix_web::{
    web::{self, Data, Json},
    HttpResponse, Scope,
};
use diesel::Connection;
use diesel::PgConnection;
use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Debug, Validate)]
pub struct RegistrationForm {
    #[validate(regex(path = "CODE_REGEX", message = "invalid code"))]
    pub workspace_code: String,

    #[validate(email)]
    pub user_email: String,

    #[validate(custom(function = "validate_password", message = "invalid passowrd"))]
    pub user_password: String,
}

pub fn routes() -> Scope {
    Scope::new("/registrations").service(web::resource("").route(web::post().to(register)))
}

async fn register(pool: Data<DbPool>, body: Json<RegistrationForm>) -> Result<HttpResponse> {
    let form = body.into_inner();
    let mut conn = pool.get().into_app_error()?;

    conn.transaction(|conn| {
        let ws = create_ws(conn, form.workspace_code)?;
        let password = create_pass(conn, form.user_password)?;
        create_user(conn, ws, password, form.user_email)?;
        Ok(())
    })
    .map(|()| HttpResponse::Ok().finish())
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

#[cfg(test)]
mod tests {

    mod registration_form {

        use crate::app::routes::registration_routes::RegistrationForm;
        use validator::Validate;

        #[test]
        fn test_valid_form() {
            let form = RegistrationForm {
                workspace_code: "CODE".to_string(),
                user_email: "eugenio@gmail.com".to_string(),
                user_password: "Eugenio@123".to_string(),
            };
            assert!(form.validate().is_ok())
        }

        #[test]
        fn test_invalid_ws_code() {
            let form = RegistrationForm {
                workspace_code: "CODE ".to_string(),
                user_email: "eugenio@gmail.com".to_string(),
                user_password: "Eugenio@123".to_string(),
            };

            let errors = form.validate().err().unwrap();
            let code_errors = *errors.field_errors().get("workspace_code").unwrap();

            assert_eq!(
                "invalid code".to_string(),
                code_errors[0].message.clone().unwrap()
            );
        }
    }
}
