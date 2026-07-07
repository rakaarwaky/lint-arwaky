use config_system_lint_arwaky::infrastructure_yaml_reader::ConfigYamlReader;

#[test]
fn test_xdg_config_dir() {
    let config_dir = dirs::config_dir();
    assert!(config_dir.is_some());
}

#[test]
fn test_config_filename() {
    assert_eq!(
        ConfigYamlReader::config_filename("rust"),
        "lint_arwaky.config.rust.yaml"
    );
    assert_eq!(
        ConfigYamlReader::config_filename("python"),
        "lint_arwaky.config.python.yaml"
    );
}
