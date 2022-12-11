fn mock_encrypter(hash: &str, verify: bool) -> Encrypter {
    let mut mock = MockEncrypter::new();
    mock.expect_encrypt().returning(|pass| Ok(hash.to_string));
    mock.expect_verify().returning(|pass, hash| Ok(verify));
    mock
}
