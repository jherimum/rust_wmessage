extern crate derive_more;

use actix_web::{
    web::{self, Data, Json},
    HttpResponse, Scope,
};
use anyhow::Result;
use diesel::Connection;
use serde::Deserialize;
use validator::Validate;

use crate::{
    config::DbPool,
    models::{password::Password, user::User, workspace::Workspace, Error},
};

use crate::commons::validators::validate_password;
use crate::commons::validators::CODE_REGEX;

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

async fn register(pool: Data<DbPool>, body: Json<RegistrationForm>) -> HttpResponse {
    let form = body.into_inner();
    let mut conn = pool.get().unwrap();

    let r: Result<()> = conn.transaction(|conn| {
        let ws = Workspace::create(conn, &form.workspace_code)?;
        let user = User::create_owner(conn, &ws, &form.user_email)?;
        let _password = Password::create(conn, &user, &form.user_password);
        Ok(())
    });

    match r {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            if let Some(e) = e.downcast_ref::<Error>() {
                return HttpResponse::Conflict().finish();
            }
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[cfg(test)]
mod tests {

    mod registration_form {

        use crate::app::routes::registrations::RegistrationForm;
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
