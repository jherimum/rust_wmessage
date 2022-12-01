use serde::Serialize;

pub mod apikey;
pub mod plugins;
pub mod registrations;

#[derive(Serialize)]
struct Response<P: Serialize> {
    status: u8,
    message: String,
    payload: P,
}

#[derive(Serialize)]
struct Error<P: Serialize> {
    r#type: String,
    title: String,
    status: u8,
    detail: String,
    instance: String,
    extensions: P,
}
