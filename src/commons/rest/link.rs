use crate::{commons::types::Result, resources::AsLink};
use actix_web::HttpRequest;
use derive_getters::Getters;
use serde::Serialize;
use url::Url;

pub const SELF_ID: &str = "self";

#[derive(Serialize, Debug, Clone, Getters)]
pub struct Link {
    #[serde(skip_serializing)]
    name: String,
    href: Url,
}

impl Link {
    pub fn new(name: &str, url: Url) -> Self {
        Link {
            name: name.to_string(),
            href: url,
        }
    }
}
