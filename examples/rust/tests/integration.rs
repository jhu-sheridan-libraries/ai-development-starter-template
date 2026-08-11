use example_pkg::greet;

#[test]
fn greet_integration_returns_expected_message() {
    assert_eq!(greet("Libraries").unwrap(), "Hello, Libraries!");
}

#[test]
fn greet_integration_rejects_empty_name() {
    assert!(greet("").is_err());
}
