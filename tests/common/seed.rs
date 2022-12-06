use diesel::{PgConnection, RunQueryDsl};

pub fn new_workspace(conn: &mut PgConnection, id: uuid::Uuid, code: &str) {
    let sql = format!(
        "INSERT INTO workspaces (id, code) VALUES ('{}', '{}')",
        id, code
    );

    diesel::sql_query(sql).execute(conn).unwrap();
}

pub fn new_workspaces(conn: &mut PgConnection, ws: Vec<(uuid::Uuid, &str)>) {
    for (id, code) in ws {
        new_workspace(conn, id, code)
    }
}

pub fn new_password(conn: &mut PgConnection, id: &uuid::Uuid, hash: &str) {
    let sql = format!(
        "INSERT INTO passwords (id, hash) VALUES ('{}', '{}')",
        id, hash
    );

    diesel::sql_query(sql).execute(conn).unwrap();
}
