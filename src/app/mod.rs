pub mod routes;

use crate::config::DbPool;
use crate::plugins::{ConnectorPlugin, ConnectorPlugins};

pub struct State {
    pub pool: DbPool,
    pub plugins: ConnectorPlugins,
}
