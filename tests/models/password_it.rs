use wmessage::models::password::Password;

use crate::{common::seed::new_password, models::build_context};

#[test]
fn test_find_pass_when_do_not_exists() {
    let ctx = build_context("test_find_pass_when_do_not_exists");
    let mut conn = ctx.build_connection_and_migrate();

    let ws = Password::find(&mut conn, &uuid::Uuid::new_v4());
    assert!(ws.is_ok() && ws.unwrap().is_none());
}

#[test]
fn test_find_pass_when_exists() {
    let ctx = build_context("test_find_pass_when_exists");
    let mut conn = ctx.build_connection_and_migrate();

    let id = uuid::Uuid::new_v4();
    new_password(&mut conn, &id, "hash");

    let pass = Password::find(&mut conn, &id);
    assert_eq!(pass.unwrap().unwrap(), Password::new(id, "hash"))
}

#[test]
fn test_pass_create() {
    let ctx = build_context("test_pass_create");
    let mut conn = ctx.build_connection_and_migrate();

    let pass = Password::create(&mut conn, "password@123").unwrap();

    assert_eq!(
        pass,
        Password::find(&mut conn, &pass.id()).unwrap().unwrap()
    );
}
