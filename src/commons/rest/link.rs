use crate::commons::types::Result;
use actix_web::HttpRequest;
use derive_getters::Getters;
use serde::Serialize;
use std::collections::HashMap;
use url::Url;

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

pub struct Links(Vec<Link>);

impl Links {
    pub fn new(links: Vec<Link>) -> Self {
        Links(links)
    }

    pub fn as_map(&self) -> HashMap<String, Link> {
        self.0
            .iter()
            .map(|l| (l.name().clone(), l.clone()))
            .collect()
    }
}

pub trait IntoLinks {
    fn to_links(&self, req: HttpRequest) -> Result<Links>;
}
