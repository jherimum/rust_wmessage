pub mod handlers;
use crate::config::DbPool;

pub struct State {
    pub pool: DbPool,
}
