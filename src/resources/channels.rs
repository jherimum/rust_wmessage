use crate::commons::error::IntoRestError;
use crate::commons::id::id::new_id;
use crate::commons::rest::link::{IntoLinks, Link, Links};
use crate::commons::rest::{AsResponse, Response, SELF_ID};
use crate::commons::types::{Code, Conn, DbPool, Id, Json, Result};
use crate::models::workspace::Workspace;
use crate::{commons::error::IntoAppError, models::channel::Channel};
use actix_web::{delete, get, patch, post, HttpRequest, Scope};
use actix_web::{
    web::{self, Data},
    HttpResponse,
};
use serde::Deserialize;

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
                "find_one_channel",
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

pub fn resources() -> Scope {
    Scope::new("/workspaces/{ws_id}/channels")
        .service(create)
        .service(delete)
        .service(update)
        .service(all)
        .service(find_one)
}

#[post("", name = "create_channel")]
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

#[get("", name = "all_channels")]
pub async fn all() -> Result<HttpResponse> {
    todo!()
}

#[get("/{channel_id}", name = "find_one_channel")]
pub async fn find_one() -> Result<HttpResponse> {
    todo!()
}

#[patch("/{channel_id}", name = "update_channel")]
pub async fn update() -> Result<HttpResponse> {
    todo!()
}

#[delete("/{channel_id}", name = "delete_channel")]
pub async fn delete() -> Result<HttpResponse> {
    todo!()
}
