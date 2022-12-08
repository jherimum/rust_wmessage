pub mod database;
pub mod encrypt;
pub mod error;
pub mod json_schema;
pub mod validators;

use uuid::Uuid;

pub fn new_uuid() -> Uuid {
    Uuid::new_v4()
}
