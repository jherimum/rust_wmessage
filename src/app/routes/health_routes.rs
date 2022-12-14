use actix_web::{
    web::{self, Data},
    HttpResponse, Scope,
};

use crate::{config::DbPool, models::health::Health};

pub fn routes() -> Scope {
    Scope::new("/health").service(web::resource("").route(web::get().to(health)))
}

async fn health(pool: Data<DbPool>) -> HttpResponse {
    match &mut pool.get() {
        Ok(conn) => match Health::up(conn) {
            Ok(_) => HttpResponse::Ok().finish(),
            Err(_) => HttpResponse::InternalServerError().finish(),
        },
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
