use serde::Serialize;
use std::{collections::HashMap, fmt::Debug};
use url::Url;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RestErrorKind {
    NotFound,
}

#[derive(Serialize, Debug, Clone)]
struct Response<T>
where
    T: Serialize + Clone + Debug,
{
    data: T,
    links: HashMap<String, Url>,
}

impl<T: Serialize + Clone + Debug> Response<T> {
    pub fn new(data: T) -> Response<T> {
        Response {
            data,
            links: HashMap::new(),
        }
    }
}
