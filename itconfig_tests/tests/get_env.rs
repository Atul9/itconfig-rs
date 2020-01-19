use std::env;
use itconfig::*;

#[test]
#[should_panic(expected = "Environment variable \"TEST_CASE_1\" is missing")]
fn missing_env_variable() {
    get_env_or_panic::<String>("TEST_CASE_1");
}

#[test]
#[should_panic(expected = "Failed to parse environment variable \"TEST_CASE_2\"")]
fn cannot_parse_env_variable() {
    env::set_var("TEST_CASE_2", "30r");
    get_env_or_panic::<u32>("TEST_CASE_2");
}

#[test]
fn get_env_successfully() {
    env::set_var("TEST_CASE_3", "30");
    let a = get_env::<u32>("TEST_CASE_3").unwrap();

    assert_eq!(a, 30);
}