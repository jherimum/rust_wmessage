use actix_web::HttpRequest;
use url::Url;

use crate::commons::{
    error::IntoAppError,
    rest::link::Link,
    types::{Id, Result},
};

pub mod apikeys;
pub mod channels;
pub mod connections;
pub mod healths;
pub mod messages;
pub mod plugins;
pub mod registrations;
pub mod workspaces;

pub enum ResourceLink {
    Channels { ws_id: Id },
    Channel { ws_id: Id, channel_id: Id },
}

impl ResourceLink {
    pub fn url(&self, req: &HttpRequest) -> Result<Url> {
        match self {
            ResourceLink::Channels { ws_id } => req
                .url_for("chanels", &[ws_id.to_string()])
                .into_app_error(),
            ResourceLink::Channel { ws_id, channel_id } => req
                .url_for("chanels", &[ws_id.to_string(), channel_id.to_string()])
                .into_app_error(),
        }
    }

    pub fn link(&self, name: &str, req: &HttpRequest) -> Result<Link> {
        Ok(Link::new(name, self.url(req)?))
    }
}
