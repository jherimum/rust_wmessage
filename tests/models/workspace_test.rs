use anyhow::Result;
use uuid::Uuid;
use wmessage::models::workspace::{Error, Workspace};

use crate::common::{seed::new_workspace, TestContext};

fn build_context(db_name: &str) -> TestContext {
    TestContext::new("postgresql://wmessage:wmessage@localhost:6543", db_name)
}

#[test]
fn test_find_ws_when_do_not_exists() {
    let ctx = build_context("test_find_ws_when_do_not_exists");
    let mut conn = ctx.build_connection_and_migrate();

    let ws = Workspace::find(&mut conn, &uuid::Uuid::new_v4());
    assert!(ws.is_ok() && ws.unwrap().is_none());
}

#[test]
fn test_find_ws_when_exists() {
    let ctx = build_context("test_find_ws_when_exists");
    let mut conn = ctx.build_connection_and_migrate();

    let id = uuid::Uuid::new_v4();
    let code = "CODE";

    new_workspace(&mut conn, id, &code);

    let ws = Workspace::find(&mut conn, &id);

    assert_eq!(ws.unwrap().unwrap(), Workspace::new(id, code));
}

#[test]
fn test_ws_creation_when_exists_ws_with_same_code() {
    let ctx = build_context("test_ws_creation_when_exists_ws_with_same_code");
    let mut conn = ctx.build_connection_and_migrate();
    let code = "code";
    new_workspace(&mut conn, Uuid::new_v4(), "code");

    let r: Result<Workspace, anyhow::Error> = Workspace::create(&mut conn, code);
    let expected_error = Error::WS001 {
        _code: code.to_string(),
    };

    match r {
        Ok(_) => assert!(false),
        Err(e) => {
            assert_eq!(expected_error, e.downcast::<Error>().unwrap());
        }
    }
}

#[test]
fn test_ws_creation_when_does_not_exists_ws_with_same_code() {
    let ctx = build_context("test_ws_creation_when_does_not_exists_ws_with_same_code");
    let mut conn = ctx.build_connection_and_migrate();

    let code = "code";

    let r: Result<Workspace, anyhow::Error> = Workspace::create(&mut conn, code);

    match r {
        Ok(ws) => assert_eq!(ws, Workspace::new(ws.id(), code)),
        Err(_) => assert!(false),
    }
}
