use crate::commons::error::IntoRestError;
use crate::commons::id::id::new_id;
use crate::commons::rest::link::{IntoLinks, Link, Links};
use crate::commons::rest::{AsResponse, Response, SELF_ID};
use crate::commons::types::{Code, Conn, DbPool, Id, Json, Result};
use crate::models::workspace::Workspace;
use crate::{commons::error::IntoAppError, models::channel::Channel};
use actix_web::HttpRequest;
use actix_web::{
    web::{self, get, patch, post, Data},
    HttpResponse, Scope,
};
use serde::Deserialize;

pub fn routes() -> Scope {
    let channels = web::resource("")
        .route(post().to(create))
        .route(get().to(all))
        .name("channels");
    let channel = web::resource("/{channel_id}")
        .route(get().to(find))
        .route(patch().to(update))
        .name("channel");

    Scope::new("/workspaces/{ws_id}/channels")
        .service(channels)
        .service(channel)
}

impl AsResponse for Channel {
    type T = Channel;

    fn to_response(self, req: HttpRequest) -> Result<Response<Self::T>> {
        Response::new(self, req)
    }
}

impl IntoLinks for Channel {
    fn to_links(&self, req: HttpRequest) -> Result<Links> {
        let mut vec = vec![];
        vec.push(Link::new(
            SELF_ID,
            req.url_for(
                "channel",
                &[self.workspace_id().to_string(), self.id().to_string()],
            )
            .into_app_error()?,
        ));
        /*
        vec.push(Link::new(
            "workspace",
            req.url_for("workspace", &[self.workspace_id().to_string()])
                .into_app_error()?,
        ));

        vec.push(Link::new(
            "message_types",
            req.url_for(
                "message_types",
                &[self.workspace_id().to_string(), self.id().to_string()],
            )
            .into_app_error()?,
        ));
         */
        Ok(Links::new(vec))
    }
}

#[derive(Deserialize, Debug, Clone)]
struct CreateChannel {
    code: Code,
    description: String,
    vars: Json,
    enabled: bool,
}

async fn create(
    pool: Data<DbPool>,
    path: web::Path<Id>,
    payload: web::Json<CreateChannel>,
    req: HttpRequest,
) -> Result<HttpResponse> {
    let mut conn = pool.get().into_app_error()?;
    let ws_id = path.into_inner();
    let workspace = retrieve_workspace(&mut conn, ws_id)?;
    let channel = build_channel(workspace, payload);
    let channel = Channel::save(&mut conn, channel)?;

    Ok(channel.to_response(req)?.ok())
}

fn retrieve_workspace(conn: &mut Conn, ws_id: Id) -> Result<Workspace> {
    Workspace::find(conn, ws_id).into_not_found("Workspace not found")
}

fn build_channel(workspace: Workspace, payload: web::Json<CreateChannel>) -> Channel {
    Channel::new(
        new_id(),
        workspace,
        payload.code.to_owned(),
        payload.description.to_owned(),
        payload.vars.clone(),
        payload.enabled,
    )
}

async fn all() -> Result<HttpResponse> {
    todo!()
}

async fn find() -> Result<HttpResponse> {
    todo!()
}

async fn update() -> Result<HttpResponse> {
    todo!()
}
