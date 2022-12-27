pub mod entity;
pub mod link;

pub const SELF_ID: &str = "self";

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RestErrorKind {
    NotFound,
}
