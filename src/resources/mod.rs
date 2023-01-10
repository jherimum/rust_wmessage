pub mod apikeys;
pub mod channels;
pub mod connections;
pub mod healths;
pub mod message_types;
pub mod messages;
pub mod plugins;
pub mod registrations;
pub mod workspaces;

use crate::commons::{
    error::IntoAppError,
    rest::link::Link,
    types::{Id, Result},
};
use actix_web::HttpRequest;
use serde::Serialize;
use url::Url;

use self::{
    channels::{CHANNELS_RESOURCE, CHANNEL_RESOURCE},
    message_types::{MESSAGE_TYPES_RESOURCE, MESSAGE_TYPE_RESOURCE},
};

pub trait AsLink {
    fn to_link(&self, name: &str, req: &HttpRequest) -> Result<Link>;
}

pub trait AsUrl {
    fn to_url(&self, req: &HttpRequest) -> Result<Url>;
}

impl AsLink for Resource {
    fn to_link(&self, name: &str, req: &HttpRequest) -> Result<Link> {
        Ok(Link::new(name, self.to_url(req)?))
    }
}

impl AsUrl for Resource {
    fn to_url(&self, req: &HttpRequest) -> Result<Url> {
        match self {
            Resource::Channels { ws_id } => req
                .url_for(CHANNELS_RESOURCE, [ws_id.to_string()])
                .into_app_error(),
            Resource::Channel { ws_id, channel_id } => req
                .url_for(
                    CHANNEL_RESOURCE,
                    [ws_id.to_string(), channel_id.to_string()],
                )
                .into_app_error(),
            Resource::MessageType {
                ws_id,
                channel_id,
                message_type_id,
            } => req
                .url_for(
                    MESSAGE_TYPE_RESOURCE,
                    [
                        ws_id.to_string(),
                        channel_id.to_string(),
                        message_type_id.to_string(),
                    ],
                )
                .into_app_error(),
            Resource::MessageTypes { ws_id, channel_id } => req
                .url_for(
                    MESSAGE_TYPES_RESOURCE,
                    [ws_id.to_string(), channel_id.to_string()],
                )
                .into_app_error(),
        }
    }
}

#[derive(Serialize, Debug, Clone)]
pub enum Resource {
    Channels {
        ws_id: Id,
    },
    Channel {
        ws_id: Id,
        channel_id: Id,
    },
    MessageType {
        ws_id: Id,
        channel_id: Id,
        message_type_id: Id,
    },
    MessageTypes {
        ws_id: Id,
        channel_id: Id,
    },
}
