use shared_lint_arwaky::config_system::taxonomy_config_vo::default_config_for_language;

#[test]
fn test_default_config_parsing() {
    let config = default_config_for_language("typescript");
    println!("typescript layers: {:?}", config.layers.keys());
    assert!(!config.layers.is_empty());
}
