pub mod apikey;
pub mod channel;
pub mod health;
pub mod message_type;
pub mod message_type_version;
pub mod password;
pub mod user;
pub mod workspace;

use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum ModelErrorKind {
    #[error("Entity not found")]
    EntityNotFound,

    #[error("Workspace with code {code} already exists")]
    WorkspaceCodeAlreadyExists { code: String },

    #[error("channel with code {code} already exists")]
    ChannelCodeAlreadyExists { code: String },
}
