pub mod routes;

use crate::config::DbPool;

pub struct State {
    pub pool: DbPool,
}
