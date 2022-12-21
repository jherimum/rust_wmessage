use wmessage::{
    commons::{encrypt::MockEncrypter, uuid::new_uuid},
    models::password::Password,
};

use crate::{common::seed::new_password, models::build_context};

#[test]
fn test_find_password_when_do_not_exists() {
    let ctx = build_context("test_find_password_when_do_not_exists");
    let mut conn = ctx.build_connection_and_migrate();

    match Password::find(&mut conn, new_uuid()) {
        Ok(None) => assert!(true),
        _ => panic!(),
    }
}

#[test]
fn test_find_password_when_exists() {
    let ctx = build_context("test_find_password_when_exists");
    let mut conn = ctx.build_connection_and_migrate();

    let id = new_uuid();

    new_password(&mut conn, &id, "hash");

    match Password::find(&mut conn, id) {
        Ok(Some(_)) => assert!(true),
        _ => panic!(),
    }
}

#[test]
fn test_pass_save() {
    let ctx = build_context("test_pass_save");
    let mut conn = ctx.build_connection_and_migrate();
    let encrypter = MockEncrypter::new();

    let id = new_uuid();

    let pass = Password::save(
        &mut conn,
        Password::new(id, "password@123", &encrypter).unwrap(),
    )
    .unwrap();

    assert_eq!(
        pass,
        Password::find(&mut conn, pass.id().clone())
            .unwrap()
            .unwrap()
    );
}
