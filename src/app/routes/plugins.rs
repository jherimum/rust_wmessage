use actix_web::{get, post, web::Data, HttpResponse, Responder};

use crate::{
    app::{self, State},
    plugins,
};

#[get("/api/plugins")]
pub async fn register(app_state: Data<State>) -> impl Responder {
    //let plugins = app_state.plugins.plugins();
    HttpResponse::Ok().finish()
}
