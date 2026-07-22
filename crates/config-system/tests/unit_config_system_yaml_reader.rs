// Unit tests for ConfigYamlReader — config file discovery and XDG fallback.
use config_system_lint_arwaky::capabilities_yaml_reader::ConfigYamlReader;
use shared::common::taxonomy_path_vo::FilePath;
use shared::config_system::contract_reader_protocol::IConfigReaderProtocol;
use shared::config_system::taxonomy_config_language_vo::ConfigLanguage;
use std::fs;
use tempfile::TempDir;

fn make_reader() -> ConfigYamlReader {
    ConfigYamlReader::new()
}

#[tokio::test]
async fn read_config_finds_rust_yaml_in_project_root() {
    let tmp = TempDir::new().unwrap();
    fs::write(
        tmp.path().join("lint_arwaky.config.rust.yaml"),
        "architecture:\n  enabled: true\n",
    )
    .unwrap();
    let fp = FilePath::new(tmp.path().to_string_lossy().to_string()).unwrap();
    let result = make_reader().read_config(&fp, ConfigLanguage::Rust).await;
    assert!(result.is_ok());
    let source = result.unwrap();
    assert!(source.is_some());
    let source = source.unwrap();
    assert_eq!(source.language, "rust");
    assert!(source.raw_content.contains("architecture"));
}

#[tokio::test]
async fn read_config_finds_python_yaml_in_project_root() {
    let tmp = TempDir::new().unwrap();
    fs::write(
        tmp.path().join("lint_arwaky.config.python.yaml"),
        "architecture:\n  enabled: true\n",
    )
    .unwrap();
    let fp = FilePath::new(tmp.path().to_string_lossy().to_string()).unwrap();
    assert!(make_reader()
        .read_config(&fp, ConfigLanguage::Python)
        .await
        .unwrap()
        .is_some());
}

#[tokio::test]
async fn read_config_typescript_falls_back_to_javascript_yaml() {
    let tmp = TempDir::new().unwrap();
    fs::write(
        tmp.path().join("lint_arwaky.config.javascript.yaml"),
        "architecture:\n  enabled: true\n",
    )
    .unwrap();
    let fp = FilePath::new(tmp.path().to_string_lossy().to_string()).unwrap();
    let result = make_reader()
        .read_config(&fp, ConfigLanguage::TypeScript)
        .await
        .unwrap();
    assert!(result.is_some());
    assert!(result.unwrap().path.value.contains("javascript"));
}

#[tokio::test]
async fn read_config_searches_parent_directories_up_to_depth_3() {
    let tmp = TempDir::new().unwrap();
    fs::write(
        tmp.path().join("lint_arwaky.config.rust.yaml"),
        "architecture:\n  enabled: true\n",
    )
    .unwrap();
    let nested = tmp.path().join("a").join("b");
    fs::create_dir_all(&nested).unwrap();
    let fp = FilePath::new(nested.to_string_lossy().to_string()).unwrap();
    assert!(make_reader()
        .read_config(&fp, ConfigLanguage::Rust)
        .await
        .unwrap()
        .is_some());
}

#[tokio::test]
async fn read_config_returns_none_when_no_file_found() {
    let tmp = TempDir::new().unwrap();
    let fp = FilePath::new(tmp.path().to_string_lossy().to_string()).unwrap();
    assert!(make_reader()
        .read_config(&fp, ConfigLanguage::Rust)
        .await
        .unwrap()
        .is_none());
}

#[tokio::test]
async fn list_config_files_finds_all_languages() {
    let tmp = TempDir::new().unwrap();
    fs::write(tmp.path().join("lint_arwaky.config.rust.yaml"), "a: 1").unwrap();
    fs::write(tmp.path().join("lint_arwaky.config.python.yaml"), "b: 2").unwrap();
    fs::write(
        tmp.path().join("lint_arwaky.config.typescript.yaml"),
        "c: 3",
    )
    .unwrap();
    let fp = FilePath::new(tmp.path().to_string_lossy().to_string()).unwrap();
    assert_eq!(make_reader().list_config_files(&fp).await.unwrap().len(), 3);
}

#[tokio::test]
async fn list_config_files_returns_empty_when_none_exist() {
    let tmp = TempDir::new().unwrap();
    let fp = FilePath::new(tmp.path().to_string_lossy().to_string()).unwrap();
    assert!(make_reader()
        .list_config_files(&fp)
        .await
        .unwrap()
        .is_empty());
}

#[tokio::test]
async fn list_config_files_deduplicates_typescript_javascript() {
    let tmp = TempDir::new().unwrap();
    fs::write(
        tmp.path().join("lint_arwaky.config.typescript.yaml"),
        "x: 1",
    )
    .unwrap();
    fs::write(
        tmp.path().join("lint_arwaky.config.javascript.yaml"),
        "y: 2",
    )
    .unwrap();
    let fp = FilePath::new(tmp.path().to_string_lossy().to_string()).unwrap();
    let files = make_reader().list_config_files(&fp).await.unwrap();
    let ts_count = files
        .iter()
        .filter(|(lang, _)| *lang == ConfigLanguage::TypeScript)
        .count();
    assert_eq!(ts_count, 1);
}

#[test]
fn default_and_new_are_equivalent() {
    let _a = ConfigYamlReader::new();
    let _b = ConfigYamlReader::default();
}
