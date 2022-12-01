extern crate derive_more;

use actix_web::{post, web::Data, HttpResponse, Responder};
use actix_web_validator::Json;
use diesel::Connection;
use serde::Deserialize;
use validator::Validate;

use crate::{
    config::DbPool,
    models::{password::Password, user::User, workspace::Workspace},
};

#[derive(Deserialize, Debug, Validate)]
pub struct RegistrationForm {
    #[validate(url)]
    pub workspace_code: String,
    pub user_email: String,
    pub user_password: String,
}

#[post("/api/registrations")]
pub async fn register(pool: Data<DbPool>, body: Json<RegistrationForm>) -> impl Responder {
    let form = body.into_inner();
    let mut conn = pool.get().unwrap();

    let x = conn.transaction::<(), anyhow::Error, _>(|conn| {
        let ws = Workspace::create(conn, &form.workspace_code)?;
        let user = User::create_owner(conn, &ws, &form.user_email)?;
        let _password = Password::create(conn, &user, &form.user_password);
        Ok(())
    });

    HttpResponse::Ok().finish()
}
