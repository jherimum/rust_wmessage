pub mod apikey;
pub mod channel;
pub mod health;
pub mod message_type;
pub mod message_type_version;
pub mod password;
pub mod user;
pub mod workspace;

use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ModelErrorKind {
    EntityNotFound { message: String },
    WorkspaceCodeAlreadyExists { code: String },
}

impl Display for ModelErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", 11111)
    }
}
