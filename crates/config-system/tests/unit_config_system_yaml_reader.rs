// Unit tests for ConfigYamlReader — config file discovery and XDG fallback.
mod common;

use config_system_lint_arwaky::capabilities_yaml_reader::ConfigYamlReader;
use shared::common::FilePath;
use shared::config_system::{ConfigLanguage, IConfigReaderProtocol};

use std::fs;
use tempfile::TempDir;

fn make_reader() -> ConfigYamlReader {
    ConfigYamlReader::new(common::make_fs())
}

#[test]
fn read_config_finds_rust_yaml_in_project_root() {
    let tmp = TempDir::new().unwrap();
    fs::write(
        tmp.path().join("lint_arwaky.config.rust.yaml"),
        "architecture:\n  enabled: true\n",
    )
    .unwrap();
    let fp = FilePath::new(tmp.path().to_string_lossy().to_string()).unwrap();
    let result = make_reader().read_config(&fp, ConfigLanguage::Rust);
    assert!(result.is_ok());
    let source = result.unwrap();
    assert!(source.is_some());
    let source = source.unwrap();
    assert_eq!(source.language, "rust");
    assert!(source.raw_content.contains("architecture"));
}

#[test]
fn read_config_finds_python_yaml_in_project_root() {
    let tmp = TempDir::new().unwrap();
    fs::write(
        tmp.path().join("lint_arwaky.config.python.yaml"),
        "architecture:\n  enabled: true\n",
    )
    .unwrap();
    let fp = FilePath::new(tmp.path().to_string_lossy().to_string()).unwrap();
    assert!(
        make_reader()
            .read_config(&fp, ConfigLanguage::Python)
            .unwrap()
            .is_some()
    );
}

#[test]
fn read_config_typescript_falls_back_to_javascript_yaml() {
    let tmp = TempDir::new().unwrap();
    fs::write(
        tmp.path().join("lint_arwaky.config.javascript.yaml"),
        "architecture:\n  enabled: true\n",
    )
    .unwrap();
    let fp = FilePath::new(tmp.path().to_string_lossy().to_string()).unwrap();
    let result = make_reader()
        .read_config(&fp, ConfigLanguage::TypeScript)
        .unwrap();
    assert!(result.is_some());
    assert!(result.unwrap().path.value.contains("javascript"));
}

#[test]
fn read_config_searches_parent_directories_up_to_depth_3() {
    let tmp = TempDir::new().unwrap();
    fs::write(
        tmp.path().join("lint_arwaky.config.rust.yaml"),
        "architecture:\n  enabled: true\n",
    )
    .unwrap();
    let nested = tmp.path().join("a").join("b");
    fs::create_dir_all(&nested).unwrap();
    let fp = FilePath::new(nested.to_string_lossy().to_string()).unwrap();
    assert!(
        make_reader()
            .read_config(&fp, ConfigLanguage::Rust)
            .unwrap()
            .is_some()
    );
}

#[test]
fn read_config_returns_none_when_no_file_found() {
    let tmp = TempDir::new().unwrap();
    let fp = FilePath::new(tmp.path().to_string_lossy().to_string()).unwrap();
    assert!(
        make_reader()
            .read_config(&fp, ConfigLanguage::Rust)
            .unwrap()
            .is_none()
    );
}

#[test]
fn list_config_files_finds_all_languages() {
    let tmp = TempDir::new().unwrap();
    fs::write(tmp.path().join("lint_arwaky.config.rust.yaml"), "a: 1").unwrap();
    fs::write(tmp.path().join("lint_arwaky.config.python.yaml"), "b: 2").unwrap();
    fs::write(
        tmp.path().join("lint_arwaky.config.typescript.yaml"),
        "c: 3",
    )
    .unwrap();
    let fp = FilePath::new(tmp.path().to_string_lossy().to_string()).unwrap();
    assert_eq!(make_reader().list_config_files(&fp).unwrap().len(), 3);
}

#[test]
fn list_config_files_returns_empty_when_none_exist() {
    let tmp = TempDir::new().unwrap();
    let fp = FilePath::new(tmp.path().to_string_lossy().to_string()).unwrap();
    assert!(make_reader().list_config_files(&fp).unwrap().is_empty());
}

#[test]
fn list_config_files_deduplicates_typescript_javascript() {
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
    let files = make_reader().list_config_files(&fp).unwrap();
    let ts_count = files
        .iter()
        .filter(|(lang, _)| *lang == ConfigLanguage::TypeScript)
        .count();
    assert_eq!(ts_count, 1);
}

#[test]
fn new_creates_instance() {
    let _a = ConfigYamlReader::new(common::make_fs());
}
