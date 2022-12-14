use crate::commons::error::AppError;
use crate::commons::error::IntoAppError;
use crate::commons::Result;
use crate::models::channel::Channel;
use crate::schema::{self, channels};
use diesel::prelude::*;
use diesel::{insert_into, PgConnection};
use schema::channels::dsl;

pub struct Channels;

impl Channels {
    pub fn exists_code(conn: &mut PgConnection, _code: &str) -> Result<bool> {
        channels::table
            .filter(dsl::code.eq(_code))
            .count()
            .get_result::<i64>(conn)
            .map(|count| count > 0)
            .into_app_error()
    }

    pub fn save(conn: &mut PgConnection, channel: Channel) -> Result<Channel> {
        if Self::exists_code(conn, &channel.code())? {
            return Err(AppError::model_error(
                crate::models::ModelErrorKind::ChannelCodeAlreadyExists {
                    code: channel.code().clone(),
                },
            ));
        }

        match insert_into(dsl::channels).values(&channel).execute(conn) {
            Ok(1) => Ok(channel),
            Ok(_) => Err(AppError::database_error("channel not inserted")),
            Err(err) => Err(AppError::from(err)),
        }
    }
}
