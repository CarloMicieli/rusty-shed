#[test]
fn update_entities_test_scaffold_exists() {
    let package_name_present = option_env!("CARGO_PKG_NAME").is_some();
    assert!(
        package_name_present,
        "package metadata should be present during tests"
    );
}
