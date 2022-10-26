use actix_web::{web, HttpResponse};
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct ChannelForm {
    code: String,
}

pub async fn create_channel(
    form: web::Json<ChannelForm>,
    path: web::Path<(String)>,
) -> HttpResponse {
    let x = path.into_inner();

    println!("{}", x);

    HttpResponse::Ok().finish()
}
