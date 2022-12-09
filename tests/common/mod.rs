pub mod seed;

use diesel::{Connection, PgConnection, RunQueryDsl};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

pub struct TestContext {
    base_url: String,
    db_name: String,
}

impl TestContext {
    pub fn build_connection_and_migrate(&self) -> PgConnection {
        let mut conn = PgConnection::establish(&format!("{}/{}", self.base_url, self.db_name))
            .unwrap_or_else(|_| panic!("Cannot connect to {} database", self.db_name));

        MigrationHarness::run_pending_migrations(&mut conn, MIGRATIONS)
            .expect("failed do run migrate");

        conn
    }

    pub fn new(base_url: &str, db_name: &str) -> Self {
        let postgres_url = format!("{}/postgres", base_url);
        let mut conn =
            PgConnection::establish(&postgres_url).expect("Cannot connect to postgres database.");

        let query = diesel::sql_query(format!("CREATE DATABASE {}", db_name).as_str());
        query
            .execute(&mut conn)
            .unwrap_or_else(|_| panic!("Could not create database {}", db_name));

        Self {
            base_url: base_url.to_string(),
            db_name: db_name.to_string(),
        }
    }
}

impl Drop for TestContext {
    fn drop(&mut self) {
        let postgres_url = format!("{}/postgres", self.base_url);
        let mut conn =
            PgConnection::establish(&postgres_url).expect("Cannot connect to postgres database.");

        let disconnect_users = format!(
            "SELECT pg_terminate_backend(pid) FROM pg_stat_activity WHERE datname = '{}';",
            self.db_name
        );

        diesel::sql_query(disconnect_users.as_str())
            .execute(&mut conn)
            .unwrap();

        let query = diesel::sql_query(format!("DROP DATABASE {}", self.db_name).as_str());
        query
            .execute(&mut conn)
            .unwrap_or_else(|_| panic!("Couldn't drop database {}", self.db_name));
    }
}
