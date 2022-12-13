pub mod database;
pub mod encrypt;
pub mod error;
pub mod json_schema;
pub mod rest;
pub mod validators;

use mockall::automock;

#[automock]
pub mod uuid {
    pub fn new_uuid() -> uuid::Uuid {
        uuid::Uuid::new_v4()
    }
}
