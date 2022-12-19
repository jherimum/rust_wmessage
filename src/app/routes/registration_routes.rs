use crate::commons::database::DbPool;
use crate::commons::error::IntoAppError;
use crate::commons::validators::validate_password;
use crate::commons::validators::CODE_REGEX;
use crate::commons::Result;
use crate::service::RegistrationService;
use actix_web::web::Data;
use actix_web::{
    web::{self, Json},
    HttpResponse, Scope,
};
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

async fn register(body: Json<RegistrationForm>, pool: Data<DbPool>) -> Result<HttpResponse> {
    let mut conn = pool.get().into_app_error()?;
    RegistrationService::register(&mut conn, body.into_inner()).map(|_| HttpResponse::Ok().finish())
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
