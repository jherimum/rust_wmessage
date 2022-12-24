pub mod link;

use self::link::{IntoLinks, Link};
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

pub trait AsResponse {
    type T: Serialize + Clone + Debug + IntoLinks;

    fn to_response(self, req: HttpRequest) -> Result<Response<Self::T>>;
}
