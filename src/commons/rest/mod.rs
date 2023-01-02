pub mod entity;
pub mod link;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RestErrorKind {
    NotFound,
}
