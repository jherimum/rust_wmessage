pub mod smtp;
use std::{collections::HashMap, ops::Deref};

use anyhow::{Context, Result};
use dyn_clone::DynClone;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

pub trait ConnectorPlugin: DynClone + Send + Sync {
    fn name(&self) -> String;
    fn properties(&self) -> Vec<Property>;
    fn dispatchers(&self) -> HashMap<DispatchType, Box<dyn DispatcherPlugin>>;
    fn dispatcher(&self, t: DispatchType) -> Option<Box<dyn DispatcherPlugin>>;
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Property {
    key: &'static str,
    description: &'static str,
    required: bool,
}

impl Property {
    pub fn new(key: &'static str, description: &'static str, required: bool) -> Self {
        Self {
            key: key,
            description: description,
            required: required,
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
