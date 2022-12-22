use actix_web::HttpRequest;
use actix_web::{http::header, HttpResponse};
use serde::Serialize;
use std::{collections::HashMap, fmt::Debug};
use url::Url;

use super::types::Result;

pub const SELF_ID: &str = "self";

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RestErrorKind {
    NotFound,
}

#[derive(Serialize, Debug, Clone)]
pub struct Response<T>
where
    T: Serialize + Clone + Debug + IntoLinks,
{
    data: T,
    links: HashMap<String, Link>,
}

impl<T: Serialize + Clone + Debug + IntoLinks> Response<T> {
    pub fn new(data: T, req: HttpRequest) -> Result<Self> {
        let l = data.to_links(req)?.as_map();
        Ok(Response {
            data: data,
            links: l,
        })
    }

    pub fn ok(&self) -> HttpResponse {
        HttpResponse::Ok().json(&self)
    }

    pub fn created(&self, location: Url) -> HttpResponse {
        HttpResponse::Created()
            .insert_header((header::LOCATION, location.to_string()))
            .json(self)
    }
}

#[derive(Serialize, Debug, Clone)]
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
        self.0.iter().map(|l| (l.name.clone(), l.clone())).collect()
    }
}

pub trait IntoLinks {
    fn to_links(&self, req: HttpRequest) -> Result<Links>;
}

pub trait AsResponse {
    type T: Serialize + Clone + Debug + IntoLinks;

    fn to_response(self, req: HttpRequest) -> Result<Response<Self::T>>;
}
