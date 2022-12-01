use wmessage::models::workspace::Workspace;

use crate::{common::TestContext, fixtures::workspace::new_workspace};

#[test]
fn test_find() {
    let ctx = TestContext::new("postgresql://wmessage:wmessage@localhost:6543", "test_find");
    let mut conn = ctx.build_connection_and_migrate();

    let id = uuid::Uuid::parse_str("30573d84-a7cc-4695-ab4a-67f316b3e4a7").unwrap();
    new_workspace(&mut conn, id, "CODE");

    let ws_not_exists = Workspace::find(&mut conn, &uuid::Uuid::new_v4());
    let ws_exists = Workspace::find(&mut conn, &id);
    assert!(ws_not_exists.is_ok() && ws_not_exists.unwrap().is_none());
    assert!(ws_exists.is_ok() && ws_exists.unwrap().is_some());
}
