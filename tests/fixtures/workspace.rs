use diesel::{PgConnection, RunQueryDsl};

pub fn new_workspace(conn: &mut PgConnection, id: uuid::Uuid, code: &str) {
    let sql = format!(
        "INSERT INTO workspaces (id, code) VALUES ('{}', '{}')",
        id, code
    );

    diesel::sql_query(sql).execute(conn).unwrap();
}
