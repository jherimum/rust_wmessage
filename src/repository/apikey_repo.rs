use crate::commons::error::AppError;
use crate::commons::Result;
use crate::models::apikey::ApiKey;
use crate::schema::api_keys;
use diesel::prelude::*;
use diesel::{insert_into, PgConnection};

pub struct ApiKeys;

impl ApiKeys {
    pub fn save(conn: &mut PgConnection, api_key: ApiKey) -> Result<ApiKey> {
        match insert_into(api_keys::table).values(&api_key).execute(conn) {
            Ok(1) => Ok(api_key),
            Ok(_) => Err(AppError::database_error("apikey not inserted")),
            Err(err) => Err(AppError::from(err)),
        }
    }
}
