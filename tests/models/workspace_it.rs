use crate::{common::seed::new_workspace, models::build_context};
use wmessage::commons::Result;
use wmessage::{commons::uuid::new_uuid, models::workspace::Workspace};

#[test]
fn test_find_ws_when_do_not_exists() {
    let ctx = build_context("test_find_ws_when_do_not_exists");
    let mut conn = ctx.build_connection_and_migrate();

    let ws = Workspace::find(&mut conn, &new_uuid());
    assert!(ws.is_ok() && ws.unwrap().is_none());
}

#[test]
fn test_find_ws_when_exists() {
    let ctx = build_context("test_find_ws_when_exists");
    let mut conn = ctx.build_connection_and_migrate();

    let id = new_uuid();
    let code = "CODE";

    new_workspace(&mut conn, id, code);

    let ws = Workspace::find(&mut conn, &id);

    assert_eq!(ws.unwrap().unwrap(), Workspace::new(code));
}

#[test]
fn test_ws_creation_when_exists_ws_with_same_code() {
    let ctx = build_context("test_ws_creation_when_exists_ws_with_same_code");
    let mut conn = ctx.build_connection_and_migrate();
    let code = "code";
    new_workspace(&mut conn, new_uuid(), "code");

    let r: Result<Workspace> = Workspace::new(code).save(&mut conn);

    match r {
        Ok(_) => assert!(true),
        Err(_) => {
            panic!()
        }
    }
}

#[test]
fn test_ws_creation_when_does_not_exists_ws_with_same_code() {
    let ctx = build_context("test_ws_creation_when_does_not_exists_ws_with_same_code");
    let mut conn = ctx.build_connection_and_migrate();

    let ws = Workspace::new("code").save(&mut conn).unwrap();
    assert_eq!(ws, Workspace::find(&mut conn, &ws.id()).unwrap().unwrap());
}
