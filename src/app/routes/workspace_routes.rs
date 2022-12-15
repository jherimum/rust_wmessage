use actix_web::{web::get, Scope};

pub fn routes() -> Scope {
    let workspaces = web::resource("")
        .route(post().to(create))
        .route(get().to(all))
        .name("channel");
    let workspace = web::resource("/{workspace_id}")
        .route(get().to(find))
        .route(patch().to(update));

    Scope::new("/workspaces")
        .service(workspace)
        .service(workspaces)
}

fn find() {}
