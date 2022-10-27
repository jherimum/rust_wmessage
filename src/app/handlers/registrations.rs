extern crate derive_more;

use std::ops::Deref;

use actix_web::{
    post,
    web::{self, Data},
    HttpResponse, Responder,
};
use diesel::Connection;

use crate::{
    app::State,
    error::AppError,
    models::{password::Password, user::User, workspace::Workspace},
    services::registration::RegistrationForm,
};

#[post("/api/registrations")]
pub async fn register(appState: Data<State>, body: web::Json<RegistrationForm>) -> impl Responder {
    let form = body.into_inner();
    let mut conn = appState.pool.get().unwrap();

    let x = conn.transaction::<(), AppError, _>(|conn| {
        let ws = Workspace::create(conn, &form.workspace_code)?;
        let user = User::create_owner(conn, &ws, &form.user_email)?;
        let _password = Password::create(conn, &user, &form.user_password);
        Ok(())
    });

    HttpResponse::Ok().finish()
}
