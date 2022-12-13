use actix_web::{
    web::{self, get, patch, post},
    HttpResponse, Scope,
};

use crate::commons::error::AppError;

pub fn routes() -> Scope {
    let channels = web::resource("")
        .route(post().to(create))
        .route(get().to(all));
    let channel = web::resource("/{channel_id}")
        .route(get().to(find))
        .route(patch().to(update));

    Scope::new("/workspaces/{ws_id}/channels")
        .service(channels)
        .service(channel)
}

async fn create() -> Result<HttpResponse, AppError> {
    todo!()
}

async fn all() -> Result<HttpResponse, AppError> {
    todo!()
}

async fn find() -> Result<HttpResponse, AppError> {
    todo!()
}

async fn update() -> Result<HttpResponse, AppError> {
    todo!()
}
