use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use serde::Deserialize;
use std::collections::HashMap;
use std::fmt::Debug;

use super::{ConnectorPlugin, DispatchType, DispatcherPlugin, Property};

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
    smtpPlugin: EmailDispatcher,
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
            smtpPlugin: EmailDispatcher::new(),
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

    fn dispatchers(&self) -> std::collections::HashMap<DispatchType, Box<dyn DispatcherPlugin>> {
        let mut map: HashMap<DispatchType, Box<dyn DispatcherPlugin>> = HashMap::new();
        map.insert(
            super::DispatchType::EMAIl,
            Box::new(self.smtpPlugin.clone()),
        );
        map
    }

    fn dispatcher(&self, t: DispatchType) -> Option<&dyn DispatcherPlugin> {
        match t {
            DispatchType::EMAIl => Some(&self.smtpPlugin),
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
    fn properties(&self) -> Vec<Property> {
        self.properties.clone()
    }

    fn dispatch(&self, req: super::Request) -> anyhow::Result<super::Response> {
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
            Ok(r) => Ok(super::Response),
            Err(e) => Err(anyhow::Error::new(e)),
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
