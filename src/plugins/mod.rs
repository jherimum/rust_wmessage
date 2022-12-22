pub mod smtp;
use crate::commons::error::IntoAppError;
use crate::commons::types::Result;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::{collections::HashMap, ops::Deref};

pub struct ConnectorPlugins {
    plugins: HashMap<String, Box<dyn ConnectorPlugin>>,
}

impl ConnectorPlugins {
    pub fn new(plugins: Vec<Box<dyn ConnectorPlugin>>) -> Self {
        let mut map = HashMap::new();

        for p in plugins {
            map.insert(p.name(), p);
        }
        ConnectorPlugins { plugins: map }
    }

    pub fn get(&self, name: String) -> Option<&dyn ConnectorPlugin> {
        self.plugins.get(&name).map(|f| f.deref())
    }

    pub fn all(&self) -> Vec<&dyn ConnectorPlugin> {
        self.plugins.values().map(|f| f.deref()).collect()
    }
}

pub trait ConnectorPlugin {
    fn name(&self) -> String;
    fn properties(&self) -> Vec<Property>;
    fn dispatchers(&self) -> HashMap<DispatchType, &dyn DispatcherPlugin>;
    fn dispatcher(&self, t: DispatchType) -> Option<&dyn DispatcherPlugin> {
        self.dispatchers().get(&t).copied()
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Eq)]
pub struct Property {
    key: &'static str,
    description: &'static str,
    required: bool,
}

impl Property {
    pub fn new(key: &'static str, description: &'static str, required: bool) -> Self {
        Self {
            key,
            description,
            required,
        }
    }
}

pub trait DispatcherPlugin {
    fn r#type(&self) -> DispatchType;
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
        serde_json::from_value::<D>(self.connector_props.clone()).into_app_error()
    }

    fn dispatcher_props<D>(&self) -> Result<D>
    where
        D: for<'a> Deserialize<'a> + DeserializeOwned,
    {
        serde_json::from_value::<D>(self.dispatcher_props.clone()).into_app_error()
    }
}

pub struct Response;

#[derive(Debug, PartialEq, Eq, Hash, Serialize)]
pub enum DispatchType {
    EMAIl,
    SMS,
    PUSH,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_property_eq() {
        assert_eq!(
            Property::new("k", "description", false),
            Property::new("k", "description", false)
        );
    }

    #[test]
    fn test_property_ne() {
        assert_ne!(
            Property::new("k", "description", true),
            Property::new("k", "description", false)
        );
    }
}
