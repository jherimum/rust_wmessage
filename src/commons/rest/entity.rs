use std::{collections::HashMap, fmt::Debug};

use actix_web::{http::header, HttpResponse};
use serde::Serialize;
use url::Url;

use crate::commons::types::Result;

use super::link::{Link, Links};

#[derive(Serialize, Debug, Clone)]
pub struct EntityModel<T>
where
    T: Serialize,
{
    data: Option<T>,
    links: HashMap<String, Link>,
}

impl<T: Serialize> EntityModel<T> {
    pub fn new(data: Option<T>, links: Links) -> Self {
        EntityModel {
            data: data,
            links: links.as_map(),
        }
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

#[derive(Serialize, Debug, Clone)]
pub struct CollectionModel<T>
where
    T: Serialize,
{
    data: Vec<EntityModel<T>>,
    links: HashMap<String, Link>,
}

impl<T: Serialize> CollectionModel<T> {
    pub fn new(data: Vec<EntityModel<T>>, links: Links) -> Self {
        CollectionModel {
            data: data,
            links: links.as_map(),
        }
    }
}
