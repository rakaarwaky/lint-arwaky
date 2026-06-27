use shared_lint_arwaky::config_system::taxonomy_config_vo::default_config_for_language;

#[test]
fn test_default_config_parsing() {
    let config = default_config_for_language("typescript");
    assert!(!config.layers.is_empty());
}

#[test]
fn test_unknown_language_returns_empty_config() {
    let config = default_config_for_language("unknown");
    assert!(config.layers.is_empty(), "unknown language should return empty config, not Rust config");
}

#[test]
fn test_rust_language_returns_rust_config() {
    let config = default_config_for_language("rust");
    assert!(!config.layers.is_empty(), "rust language should return non-empty config");
}
