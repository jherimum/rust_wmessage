use crate::commons::error::AppError;
use crate::commons::error::IntoAppError;
use crate::schema::health::dsl::*;
use diesel::prelude::*;
use diesel::PgConnection;

#[derive(Queryable, Debug, Clone, PartialEq, Eq)]
#[diesel(table_name = health)]
pub struct Health {
    id: i32,
}

impl Health {
    pub fn up(conn: &mut PgConnection) -> Result<(), AppError> {
        health
            .first::<Health>(conn)
            .optional()
            .map(|_| ())
            .into_app_error()
    }
}
