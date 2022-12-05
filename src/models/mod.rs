pub mod apikey;
pub mod channel;
pub mod password;
pub mod user;
pub mod workspace;

use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum Error {
    #[error("A workspace with code {code} aleready exists")]
    WS001 { code: String },
}
