use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RestErrorKind {
    NotFound,
}

#[derive(Serialize, Debug, Clone)]
struct Payload<T>
where
    T: Serialize,
{
    data: T,
}
