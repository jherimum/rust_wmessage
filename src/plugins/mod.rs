pub mod smtp;
use std::{collections::HashMap, ops::Deref};

use anyhow::{Context, Result};
use serde::{de::DeserializeOwned, Deserialize};

pub struct ConnectorPlugins {
    pub plugins: HashMap<String, Box<dyn ConnectorPlugin>>,
}

impl ConnectorPlugins {
    pub fn new(plugins: Vec<Box<dyn ConnectorPlugin>>) -> Self {
        let mut map: HashMap<String, Box<dyn ConnectorPlugin>> = HashMap::new();

        for b in plugins {
            map.insert(b.name().clone(), b);
        }

        ConnectorPlugins { plugins: map }
    }

    pub fn get(&self, name: String) -> Option<&dyn ConnectorPlugin> {
        self.plugins.get(&name).map(|p| p.deref())
    }
}

pub trait ConnectorPlugin {
    fn name(&self) -> String;
    fn properties(&self) -> Vec<Property>;
    fn dispatchers(&self) -> HashMap<DispatchType, Box<dyn DispatcherPlugin>>;
}

#[derive(Debug, Clone)]
pub struct Property {
    key: String,
    description: String,
    required: bool,
}

impl Property {
    pub fn new(key: String, description: String, required: bool) -> Self {
        Self {
            key: key,
            description: description,
            required: required,
        }
    }
}

pub trait DispatcherPlugin {
    fn dispatch(&self, req: Request) -> Result<Response>;
    fn properties(&self) -> Vec<Property>;
}

pub struct Request {
    id: uuid::Uuid,
    connector_props: serde_json::Value,
    dispatcher_props: serde_json::Value,
    payload: serde_json::Value,
}

impl Request {
    fn connector_props<D>(&self) -> Result<D>
    where
        D: for<'a> Deserialize<'a> + DeserializeOwned,
    {
        serde_json::from_value::<D>(self.connector_props.clone())
            .context("error while deserealization")
    }

    fn dispatcher_props<D>(&self) -> Result<D>
    where
        D: for<'a> Deserialize<'a> + DeserializeOwned,
    {
        serde_json::from_value::<D>(self.dispatcher_props.clone())
            .context("error while deserealization")
    }
}

pub struct Response;

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum DispatchType {
    EMAIl,
    SMS,
    PUSH,
}
