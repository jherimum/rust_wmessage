use super::{ConnectorPlugin, DispatchType, DispatcherPlugin, Property};
use crate::commons::error::AppError;
use crate::commons::Result;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use serde::Deserialize;
use std::collections::HashMap;
use std::fmt::Debug;

const MAIL_SMTP_HOST: &str = "mail.smtp.host";
const MAIL_SMTP_PORT: &str = "mail.smtp.port";
const MAIL_SMTP_AUTH_USERNAME: &str = "mail.smtp.auth.username";
const MAIL_SMTP_AUTH_PASSWORD: &str = "mail.smtp.auth.password";
const MAIL_SMTP_FROM: &str = "mail.smtp.from";

#[derive(Debug, Clone, Deserialize)]
pub struct ConnectorProperties {
    #[serde(rename(deserialize = "mail.smtp.host"))]
    host: String,

    #[serde(rename(deserialize = "mail.smtp.port"))]
    port: u16,

    #[serde(rename(deserialize = "mail.smtp.auth.username"))]
    username: String,

    #[serde(rename(deserialize = "mail.smtp.auth.password"))]
    password: String,
}

#[derive(Clone)]
pub struct StmpPlugin {
    properties: Vec<Property>,
    smtp_plugin: EmailDispatcher,
}

impl Default for StmpPlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl StmpPlugin {
    pub fn new() -> Self {
        StmpPlugin {
            properties: vec![
                Property::new(MAIL_SMTP_HOST, "host", true),
                Property::new(MAIL_SMTP_PORT, "port", true),
                Property::new(MAIL_SMTP_AUTH_USERNAME, "username", true),
                Property::new(MAIL_SMTP_AUTH_PASSWORD, "password", true),
            ],
            smtp_plugin: EmailDispatcher::new(),
        }
    }
}

impl ConnectorPlugin for StmpPlugin {
    fn name(&self) -> String {
        "smtp".to_string()
    }

    fn properties(&self) -> Vec<Property> {
        self.properties.clone()
    }

    fn dispatchers(&self) -> std::collections::HashMap<DispatchType, &dyn DispatcherPlugin> {
        let mut map: HashMap<DispatchType, &dyn DispatcherPlugin> = HashMap::new();
        map.insert(super::DispatchType::EMAIl, &self.smtp_plugin);
        map
    }

    fn dispatcher(&self, t: DispatchType) -> Option<&dyn DispatcherPlugin> {
        match t {
            DispatchType::EMAIl => Some(&self.smtp_plugin),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
struct DispatcherProprepties {
    #[serde(rename(deserialize = "mail.smtp.from"))]
    from: String,
}

#[derive(Debug, Clone)]
struct EmailDispatcher {
    properties: Vec<Property>,
}

impl EmailDispatcher {
    fn new() -> Self {
        EmailDispatcher {
            properties: vec![Property::new(MAIL_SMTP_FROM, "from email", true)],
        }
    }
}

impl DispatcherPlugin for EmailDispatcher {
    fn r#type(&self) -> DispatchType {
        DispatchType::EMAIl
    }

    fn properties(&self) -> Vec<Property> {
        self.properties.clone()
    }

    fn dispatch(&self, req: super::Request) -> Result<super::Response> {
        let conn_props: ConnectorProperties = req.connector_props()?;
        let disp_props: DispatcherProprepties = req.dispatcher_props()?;

        let email = Message::builder()
            .from(disp_props.from.parse().unwrap())
            .reply_to("Yuin <yuin@domain.tld>".parse().unwrap())
            .to("Hei <hei@domain.tld>".parse().unwrap())
            .subject("Happy new year")
            .body(String::from("Be happy!"))
            .unwrap();

        let creds = Credentials::new(conn_props.username, conn_props.password);

        let mailer = SmtpTransport::relay(&conn_props.host)
            .unwrap()
            .credentials(creds)
            .port(conn_props.port)
            .build();

        let x = mailer.send(&email);

        match x {
            Ok(_) => Ok(super::Response),
            Err(e) => Err(AppError::from(e)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_smtp() {
        let p = StmpPlugin::new();
        assert_eq!(p.properties().len(), 4);
    }
}
