pub mod link;
pub mod models;

use self::link::{IntoLinks, Link, Links};
use super::types::Result;
use actix_web::HttpRequest;
use actix_web::{http::header, HttpResponse};
use serde::Serialize;
use std::{collections::HashMap, fmt::Debug};
use url::Url;

pub const SELF_ID: &str = "self";

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RestErrorKind {
    NotFound,
}

#[derive(Serialize, Debug, Clone)]
pub struct EntityModel<T>
where
    T: Serialize + Clone + Debug + IntoLinks,
{
    data: T,
    links: HashMap<String, Link>,
}

impl<T: Serialize + Clone + Debug + IntoLinks> EntityModel<T> {
    pub fn new(data: T, links: Links) -> Result<Self> {
        Ok(EntityModel {
            data: data,
            links: links.as_map(),
        })
    }

    pub fn ok(&self) -> HttpResponse {
        HttpResponse::Ok().json(&self)
    }

    pub fn created(&self, location: Option<Url>) -> HttpResponse {
        let mut r = HttpResponse::Created();

        if let Some(url) = location {
            r.insert_header((header::LOCATION, url.to_string()));
        }

        r.json(&self.data)
    }
}

pub trait AsResponse {
    type T: Serialize + Clone + Debug + IntoLinks;

    fn to_response(self, req: HttpRequest) -> Result<EntityModel<Self::T>>;
}
