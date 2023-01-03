use super::link::Link;
use crate::commons::types::Result;
use actix_web::{http::header, HttpRequest, HttpResponse};
use serde::Serialize;
use std::{collections::HashMap, fmt::Debug};
use url::Url;

pub trait IntoEntityModel<T: Serialize> {
    fn to_entity_model(&self, req: &HttpRequest) -> Result<EntityModel<T>>;
}

#[derive(Serialize, Debug, Clone)]
pub struct EntityModel<T>
where
    T: Serialize,
{
    data: Option<T>,
    links: HashMap<String, Link>,
}

impl<T: Serialize> EntityModel<T> {
    pub fn new() -> Self {
        EntityModel {
            data: None,
            links: HashMap::new(),
        }
    }

    pub fn with_data(&mut self, data: T) -> &mut Self {
        self.data = Some(data);
        self
    }

    pub fn with_link(&mut self, link: Link) -> &mut Self {
        self.links.insert(link.name().to_string(), link);
        self
    }

    pub fn with_links(&mut self, links: Vec<Link>) -> &mut Self {
        links.iter().for_each(|l| {
            self.with_link(l.clone());
        });
        self
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
    pub fn new(data: Vec<EntityModel<T>>) -> Self {
        CollectionModel {
            data: data,
            links: HashMap::new(),
        }
    }
}
