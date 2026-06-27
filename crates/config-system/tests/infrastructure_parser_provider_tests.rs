use config_system_lint_arwaky::infrastructure_parser_provider::ConfigParserProvider;
use shared::config_system::contract_parser_port::IConfigParserPort;
use shared::common::taxonomy_path_vo::FilePath;
use std::io::Write;

fn temp_dir() -> std::path::PathBuf {
    let p = std::env::temp_dir().join(format!(
        "config_parser_test_{}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    std::fs::create_dir_all(&p).unwrap();
    p
}

// ─── parse_yaml_config ─────────────────────────────────────────────────────

#[test]
fn parses_valid_yaml_config() {
    let dir = temp_dir();
    let yaml_path = dir.join("lint_arwaky.yaml");
    let mut f = std::fs::File::create(&yaml_path).unwrap();
    writeln!(f, "project_name: test_project").unwrap();
    writeln!(f, "thresholds:").unwrap();
    writeln!(f, "  score: 90.0").unwrap();
    writeln!(f, "  complexity: 5").unwrap();
    writeln!(f, "  max_file_lines: 300").unwrap();
    f.flush().unwrap();

    let parser = ConfigParserProvider::new();
    let fp = FilePath::new(yaml_path.to_string_lossy().to_string()).unwrap();
    let result = parser.parse_yaml_config(&fp);
    assert!(result.is_ok(), "expected Ok, got: {:?}", result.err());
    let config = result.unwrap();
    assert_eq!(config.project_name.value, "test_project");
}

#[test]
fn nonexistent_yaml_file_returns_error() {
    let parser = ConfigParserProvider::new();
    let fp = FilePath::new("/nonexistent/path/config.yaml".to_string()).unwrap();
    let result = parser.parse_yaml_config(&fp);
    assert!(result.is_err());
}

#[test]
fn invalid_yaml_content_returns_error() {
    let dir = temp_dir();
    let yaml_path = dir.join("bad.yaml");
    let mut f = std::fs::File::create(&yaml_path).unwrap();
    writeln!(f, "{{ invalid yaml: broken :: }").unwrap();
    f.flush().unwrap();

    let parser = ConfigParserProvider::new();
    let fp = FilePath::new(yaml_path.to_string_lossy().to_string()).unwrap();
    let result = parser.parse_yaml_config(&fp);
    assert!(result.is_err());
}

#[test]
fn parses_yaml_with_adapters() {
    let dir = temp_dir();
    let yaml_path = dir.join("lint_arwaky.yaml");
    let mut f = std::fs::File::create(&yaml_path).unwrap();
    writeln!(f, "adapters:").unwrap();
    writeln!(f, "  - name: ruff").unwrap();
    writeln!(f, "    status: disabled").unwrap();
    writeln!(f, "  - name: mypy").unwrap();
    writeln!(f, "    status: enabled").unwrap();
    f.flush().unwrap();

    let parser = ConfigParserProvider::new();
    let fp = FilePath::new(yaml_path.to_string_lossy().to_string()).unwrap();
    let result = parser.parse_yaml_config(&fp);
    assert!(result.is_ok());
    let config = result.unwrap();
    assert_eq!(config.adapters.len(), 2);
    assert_eq!(config.adapters[0].name.to_string(), "ruff");
}

// ─── parse_toml_config ─────────────────────────────────────────────────────

#[test]
fn parses_valid_toml_config() {
    let dir = temp_dir();
    let toml_path = dir.join("Cargo.toml");
    let mut f = std::fs::File::create(&toml_path).unwrap();
    writeln!(f, "[package]").unwrap();
    writeln!(f, "name = \"test_pkg\"").unwrap();
    writeln!(f, "[tool.lint_arwaky]").unwrap();
    writeln!(f, "project_name = \"toml_project\"").unwrap();
    writeln!(f, "[tool.lint_arwaky.thresholds]").unwrap();
    writeln!(f, "score = 85.0").unwrap();
    writeln!(f, "complexity = 8").unwrap();
    writeln!(f, "max_file_lines = 400").unwrap();
    f.flush().unwrap();

    let parser = ConfigParserProvider::new();
    let fp = FilePath::new(toml_path.to_string_lossy().to_string()).unwrap();
    let result = parser.parse_toml_config(&fp);
    assert!(result.is_some());
    assert!(result.as_ref().unwrap().is_ok());
    let config = result.unwrap().unwrap();
    assert_eq!(config.project_name.value, "toml_project");
}

#[test]
fn nonexistent_toml_file_returns_some_err() {
    let parser = ConfigParserProvider::new();
    let fp = FilePath::new("/nonexistent/path/Cargo.toml".to_string()).unwrap();
    let result = parser.parse_toml_config(&fp);
    assert!(result.is_some());
    assert!(result.as_ref().unwrap().is_err());
}

#[test]
fn toml_without_tool_lint_arwaky_returns_none() {
    let dir = temp_dir();
    let toml_path = dir.join("Cargo.toml");
    let mut f = std::fs::File::create(&toml_path).unwrap();
    writeln!(f, "[package]").unwrap();
    writeln!(f, "name = \"simple\"").unwrap();
    writeln!(f, "version = \"1.0.0\"").unwrap();
    f.flush().unwrap();

    let parser = ConfigParserProvider::new();
    let fp = FilePath::new(toml_path.to_string_lossy().to_string()).unwrap();
    let result = parser.parse_toml_config(&fp);
    assert!(result.is_none());
}

#[test]
fn invalid_toml_content_returns_some_err() {
    let dir = temp_dir();
    let toml_path = dir.join("Cargo.toml");
    let mut f = std::fs::File::create(&toml_path).unwrap();
    writeln!(f, "[[[invalid]]]").unwrap();
    f.flush().unwrap();

    let parser = ConfigParserProvider::new();
    let fp = FilePath::new(toml_path.to_string_lossy().to_string()).unwrap();
    let result = parser.parse_toml_config(&fp);
    assert!(result.is_some());
    assert!(result.as_ref().unwrap().is_err());
}
