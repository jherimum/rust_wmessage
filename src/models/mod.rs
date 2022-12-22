pub mod apikey;
pub mod channel;
pub mod health;
pub mod message;
pub mod message_type;
pub mod message_type_version;
pub mod password;
pub mod user;
pub mod workspace;
use thiserror::Error;

use crate::commons::{
    error::{AppError, AppErrorKind},
    types::Result,
};

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum ModelErrorKind {
    #[error("Entity not found")]
    EntityNotFound,

    #[error("Workspace with code {code} already exists")]
    WorkspaceCodeAlreadyExists { code: String },

    #[error("channel with code {code} already exists")]
    ChannelCodeAlreadyExists { code: String },
}

impl ModelErrorKind {
    fn entity_not_found(message: &str) -> AppError {
        AppError::new(
            AppErrorKind::ModelError(ModelErrorKind::EntityNotFound),
            message,
            None,
        )
    }
}

pub trait IntoEntityNotFound<T> {
    fn into_entity_not_found(self, message: &str) -> Result<T>;
}

impl<T> IntoEntityNotFound<T> for Option<T> {
    fn into_entity_not_found(self, message: &str) -> Result<T> {
        self.ok_or_else(|| ModelErrorKind::entity_not_found(message))
    }
}
