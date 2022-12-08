use crate::commons::error::AppError;
use crate::schema::health::dsl::*;
use diesel::prelude::*;
use diesel::PgConnection;

#[derive(Queryable, Debug, Clone, PartialEq)]
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
            .map_err(|err| AppError::from(err))
    }
}
