use shared_lint_arwaky::taxonomy_adapter_name_vo::AdapterName;

#[test]
fn test_adapter_name_new() {
    let name = AdapterName::new("ruff").unwrap_or_default();
    assert_eq!(name.value, "ruff");

    let name = AdapterName::new("  ruff  ").unwrap_or_default();
    assert_eq!(name.value, "ruff");

    let name = AdapterName::new("my adapter").unwrap_or_default();
    assert_eq!(name.value, "my adapter");
}

#[test]
fn test_adapter_name_invalid() {
    assert!(AdapterName::new("").is_err());
    assert!(AdapterName::new("   ").is_err());
    assert!(AdapterName::new("\t\n  ").is_err());
}
