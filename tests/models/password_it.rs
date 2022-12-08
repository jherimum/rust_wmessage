use wmessage::{commons::encrypt::MockEncrypter, models::password::Password};

use crate::{common::seed::new_password, models::build_context};

#[test]
fn test_find_password_when_do_not_exists() {
    let ctx = build_context("test_find_password_when_do_not_exists");
    let mut conn = ctx.build_connection_and_migrate();

    match Password::find(&mut conn, &uuid::Uuid::new_v4()) {
        Ok(None) => assert!(true),
        _ => assert!(false),
    }
}

#[test]
fn test_find_password_when_exists() {
    let ctx = build_context("test_find_password_when_exists");
    let mut conn = ctx.build_connection_and_migrate();

    let id = uuid::Uuid::new_v4();

    new_password(&mut conn, &id, "hash");

    match Password::find(&mut conn, &id) {
        Ok(Some(_)) => assert!(true),
        _ => assert!(false),
    }
}

#[test]
fn test_pass_save() {
    let ctx = build_context("test_pass_save");
    let mut conn = ctx.build_connection_and_migrate();
    let encrypter = MockEncrypter::new();

    let pass = Password::new("password@123", &encrypter)
        .unwrap()
        .save(&mut conn)
        .unwrap();

    assert_eq!(
        pass,
        Password::find(&mut conn, &pass.id()).unwrap().unwrap()
    );
}
