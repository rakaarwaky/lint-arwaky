use config_system_lint_arwaky::infrastructure_yaml_reader::ConfigYamlReader;

#[test]
fn test_xdg_config_dir() {
    let config_dir = dirs::config_dir();
    assert!(config_dir.is_some());
}

#[test]
fn test_xdg_directories() {
    let data_dir = ConfigYamlReader::data_dir();
    assert!(data_dir.is_some());
    let data_path = data_dir.expect("data_dir should be set");
    assert!(data_path.ends_with("lint-arwaky"));

    let cache_dir = ConfigYamlReader::cache_dir();
    assert!(cache_dir.is_some());
    let cache_path = cache_dir.expect("cache_dir should be set");
    assert!(cache_path.ends_with("lint-arwaky"));

    let state_dir = ConfigYamlReader::state_dir();
    assert!(state_dir.is_some());
    let state_path = state_dir.expect("state_dir should be set");
    assert!(state_path.ends_with("lint-arwaky"));
}

#[test]
fn test_config_filename() {
    assert_eq!(ConfigYamlReader::config_filename("rust"), "lint_arwaky.config.rust.yaml");
    assert_eq!(ConfigYamlReader::config_filename("python"), "lint_arwaky.config.python.yaml");
}
