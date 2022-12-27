use std::{collections::HashMap, fmt::Debug};

use actix_web::{http::header, HttpRequest, HttpResponse};
use serde::Serialize;
use url::Url;

use crate::commons::types::Result;

use super::link::{IntoLinks, Link, Links};

#[derive(Serialize, Debug, Clone)]
pub struct EntityModel<T>
where
    T: Serialize + Clone + Debug + IntoLinks,
{
    data: Option<T>,
    links: HashMap<String, Link>,
}

impl<T: Serialize + Clone + Debug + IntoLinks> EntityModel<T> {
    pub fn new(data: Option<T>, links: Links) -> Result<Self> {
        Ok(EntityModel {
            data: data,
            links: links.as_map(),
        })
    }

    pub fn ok(&self) -> Result<HttpResponse> {
        Ok(HttpResponse::Ok().json(&self))
    }

    pub fn created(&self, location: Option<Url>) -> Result<HttpResponse> {
        let mut r = HttpResponse::Created();

        if let Some(url) = location {
            r.insert_header((header::LOCATION, url.to_string()));
        }

        Ok(r.json(&self))
    }
}

pub trait AsResponse {
    type T: Serialize + Clone + Debug + IntoLinks;

    fn to_response(&self, req: &HttpRequest) -> Result<EntityModel<Self::T>>;
}
