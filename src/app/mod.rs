pub mod routes;

use std::collections::HashMap;

use crate::config::DbPool;
use crate::plugins::ConnectorPlugin;

pub struct State {
    pub pool: DbPool,
    pub plugins: HashMap<String, Box<dyn ConnectorPlugin>>,
}
