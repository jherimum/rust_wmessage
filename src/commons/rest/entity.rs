use super::link::{IntoLinks, Link};
use crate::commons::types::Result;
use actix_web::{http::header, HttpRequest, HttpResponse};
use serde::Serialize;
use std::{collections::HashMap, fmt::Debug};
use url::Url;

pub trait Entity
where
    Self: LinkHolder + Serialize,
{
    fn with_link(&mut self, link: Link) -> Result<&mut Self> {
        self.links().insert(link.name().to_string(), link);
        Ok(self)
    }

    fn with_links(&mut self, links: impl IntoLinks, req: &HttpRequest) -> Result<&mut Self> {
        for l in links.to_links(req)? {
            self.with_link(l.clone())?;
        }
        Ok(self)
    }

    fn ok(&self) -> Result<HttpResponse> {
        Ok(HttpResponse::Ok().json(self))
    }

    fn created(&self, location: Option<Url>) -> Result<HttpResponse> {
        let mut r = HttpResponse::Created();

        if let Some(url) = location {
            r.insert_header((header::LOCATION, url.to_string()));
        }

        Ok(r.json(self))
    }
}

pub trait LinkHolder {
    fn links(&mut self) -> &mut HashMap<String, Link>;
}

impl<T: Serialize> LinkHolder for EntityModel<T> {
    fn links(&mut self) -> &mut HashMap<String, Link> {
        &mut self.links
    }
}

impl<T: Serialize> LinkHolder for CollectionModel<T> {
    fn links(&mut self) -> &mut HashMap<String, Link> {
        &mut self.links
    }
}

impl<T: Serialize> Entity for EntityModel<T> {}
impl<T: Serialize> Entity for CollectionModel<T> {}

pub trait ToEntityModel<T: Serialize> {
    fn to_entity_model(&self, req: &HttpRequest) -> Result<EntityModel<T>>;
}

pub trait ToCollectionModel<T: Serialize> {
    fn to_collection_model(&self, req: &HttpRequest) -> Result<CollectionModel<T>>;
}

#[derive(Serialize, Debug, Clone)]
pub struct EntityModel<T>
where
    T: Serialize,
{
    data: Option<T>,
    links: HashMap<String, Link>,
}

impl<T: Serialize> Default for EntityModel<T> {
    fn default() -> Self {
        Self::new()
    }
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
}

#[derive(Serialize, Debug, Clone)]
pub struct CollectionModel<T>
where
    T: Serialize,
{
    data: Vec<EntityModel<T>>,
    links: HashMap<String, Link>,
}

impl<T: Serialize> Default for CollectionModel<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Serialize> CollectionModel<T> {
    pub fn new() -> Self {
        CollectionModel {
            data: vec![],
            links: HashMap::new(),
        }
    }

    pub fn add_entity(&mut self, entity: EntityModel<T>) -> &mut Self {
        self.data.push(entity);
        self
    }

    pub fn add_entities(&mut self, entities: Vec<EntityModel<T>>) -> &mut Self {
        self.data.extend(entities);
        self
    }
}
