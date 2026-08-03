// FR-010 — Config File Listing
// Tests listing all config files found at project root for all supported languages.
mod common;

use config_system_lint_arwaky::capabilities_yaml_reader::ConfigYamlReader;
use shared::common::FilePath;
use shared::config_system::IConfigReaderProtocol;
use std::fs;
use tempfile::TempDir;

fn make_reader() -> ConfigYamlReader {
    ConfigYamlReader::new(common::make_fs())
}

// FR-010: Multiple languages have config files → all returned
#[test]
fn us10_lists_config_files_for_all_languages() {
    let tmp = TempDir::new().unwrap();
    fs::write(tmp.path().join("lint_arwaky.config.rust.yaml"), "a: 1").unwrap();
    fs::write(tmp.path().join("lint_arwaky.config.python.yaml"), "b: 2").unwrap();
    fs::write(
        tmp.path().join("lint_arwaky.config.typescript.yaml"),
        "c: 3",
    )
    .unwrap();
    let fp = FilePath::new(tmp.path().to_string_lossy().to_string()).unwrap();
    let files = make_reader().list_config_files(&fp).unwrap();
    assert_eq!(files.len(), 3);
}

// FR-010: No config files for any language → returns empty list
#[test]
fn us10_returns_empty_when_no_configs_exist() {
    let tmp = TempDir::new().unwrap();
    let fp = FilePath::new(tmp.path().to_string_lossy().to_string()).unwrap();
    assert!(make_reader().list_config_files(&fp).unwrap().is_empty());
}

// FR-010: TypeScript and JavaScript configs deduplicated
#[test]
fn us10_deduplicates_typescript_javascript_configs() {
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
    // Should only list TypeScript once (first found per language)
    let ts_count = files
        .iter()
        .filter(|(lang, _)| *lang == shared::config_system::ConfigLanguage::TypeScript)
        .count();
    assert_eq!(ts_count, 1);
}

// FR-010: Only Rust config exists → single entry
#[test]
fn us10_lists_single_rust_config() {
    let tmp = TempDir::new().unwrap();
    fs::write(tmp.path().join("lint_arwaky.config.rust.yaml"), "a: 1").unwrap();
    let fp = FilePath::new(tmp.path().to_string_lossy().to_string()).unwrap();
    let files = make_reader().list_config_files(&fp).unwrap();
    assert_eq!(files.len(), 1);
    assert_eq!(
        files[0].0,
        shared::config_system::ConfigLanguage::Rust
    );
}
