use super::link::Link;
use crate::commons::types::Result;
use actix_web::HttpRequest;
use serde::Serialize;
use std::{collections::HashMap, fmt::Debug};

pub trait IntoSimpleEntity<T> {
    fn to_simple_entity(&self, req: &HttpRequest) -> Result<SimpleEntity<T>>;
}

#[derive(Debug, Serialize, Clone)]
pub struct SimpleEntity<T> {
    data: Option<T>,
    links: HashMap<String, Link>,
}

impl<T> SimpleEntity<T> {
    pub fn new(data: Option<T>, links: HashMap<String, Link>) -> Self {
        SimpleEntity { data, links }
    }
}

pub trait IntoSimpleEntityCollection<T> {
    fn to_simple_entity_collection(&self, req: &HttpRequest) -> Result<SimpleEntityCollection<T>>;
}

#[derive(Debug, Serialize, Clone)]
pub struct SimpleEntityCollection<T> {
    data: Vec<SimpleEntity<T>>,
    links: HashMap<String, Link>,
}

impl<T> SimpleEntityCollection<T> {
    pub fn new(data: Vec<SimpleEntity<T>>, links: HashMap<String, Link>) -> Self {
        SimpleEntityCollection { data, links }
    }
}
