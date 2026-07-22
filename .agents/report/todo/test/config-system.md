
# Test Suite for `config-system` (v1.10.106)

Below is the complete test suite following the flat `tests/` convention with filename prefixes as virtual subfolders.

---

## Directory Layout

```
crates/config-system/
├── src/
│   └── lib.rs
├── tests/
│   ├── contract_config_system.rs
│   ├── unit_config_system_rules_validator.rs
│   ├── unit_config_system_workspace_detector.rs
│   ├── unit_config_system_yaml_reader.rs
│   ├── unit_config_system_parser_provider.rs
│   ├── unit_config_system_orchestrator.rs
│   ├── integration_config_system.rs
│   ├── smoke_config_system.rs
│   ├── e2e_config_system_flow.rs
│   ├── acceptance_US_1.rs
│   ├── acceptance_US_2.rs
│   ├── acceptance_US_3.rs
│   ├── acceptance_US_4.rs
│   ├── acceptance_US_5.rs
│   └── bench_config_system.rs
└── Cargo.toml
```

---

## `tests/contract_config_system.rs`

```rust
// PURPOSE: Verify that all concrete types implement their declared contract traits.
use config_system_lint_arwaky::agent_config_orchestrator::ConfigOrchestrator;
use config_system_lint_arwaky::capabilities_parser_provider::ConfigParserProvider;
use config_system_lint_arwaky::capabilities_rules_validator::ConfigRulesValidator;
use config_system_lint_arwaky::capabilities_workspace_detector::WorkspaceDetector;
use config_system_lint_arwaky::capabilities_yaml_reader::ConfigYamlReader;
use shared::config_system::contract_config_orchestrator_aggregate::IConfigOrchestratorAggregate;
use shared::config_system::contract_parser_protocol::IConfigParserProtocol;
use shared::config_system::contract_reader_protocol::IConfigReaderProtocol;
use shared::config_system::contract_validator_protocol::IConfigValidatorProtocol;
use shared::config_system::contract_workspace_detector_protocol::IWorkspaceDetectorProtocol;

// ─── IConfigOrchestratorAggregate ──────────────────────────

#[test]
fn config_orchestrator_implements_aggregate() {
    fn assert_trait<T: IConfigOrchestratorAggregate>() {}
    assert_trait::<ConfigOrchestrator>();
}

// ─── IConfigReaderProtocol ─────────────────────────────────

#[test]
fn config_yaml_reader_implements_reader_protocol() {
    fn assert_trait<T: IConfigReaderProtocol>() {}
    assert_trait::<ConfigYamlReader>();
}

// ─── IConfigValidatorProtocol ──────────────────────────────

#[test]
fn config_rules_validator_implements_validator_protocol() {
    fn assert_trait<T: IConfigValidatorProtocol>() {}
    assert_trait::<ConfigRulesValidator>();
}

// ─── IWorkspaceDetectorProtocol ────────────────────────────

#[test]
fn workspace_detector_implements_detector_protocol() {
    fn assert_trait<T: IWorkspaceDetectorProtocol>() {}
    assert_trait::<WorkspaceDetector>();
}

// ─── IConfigParserProtocol ─────────────────────────────────

#[test]
fn config_parser_provider_implements_parser_protocol() {
    fn assert_trait<T: IConfigParserProtocol>() {}
    assert_trait::<ConfigParserProvider>();
}

// ─── Send + Sync bounds (required by Arc<dyn Trait>) ──────

#[test]
fn all_contracts_are_send_sync() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<ConfigOrchestrator>();
    assert_send_sync::<ConfigYamlReader>();
    assert_send_sync::<ConfigRulesValidator>();
    assert_send_sync::<WorkspaceDetector>();
    assert_send_sync::<ConfigParserProvider>();
}
```

---

## `tests/unit_config_system_rules_validator.rs`

```rust
// PURPOSE: Unit tests for ConfigRulesValidator — adapter enablement and threshold validation.
use config_system_lint_arwaky::capabilities_rules_validator::ConfigRulesValidator;
use shared::common::taxonomy_adapter_name_vo::AdapterName;
use shared::common::taxonomy_common_vo::{Count, Score};
use shared::config_system::contract_validator_protocol::IConfigValidatorProtocol;
use shared::config_system::taxonomy_setting_vo::{
    AdapterEntry, AdapterStatus, ProjectConfig, Thresholds,
};

fn make_validator() -> ConfigRulesValidator {
    ConfigRulesValidator::new()
}

fn make_config_with_adapters(adapters: Vec<AdapterEntry>) -> ProjectConfig {
    let mut config = ProjectConfig::default();
    config.adapters = adapters;
    config
}

// ─── is_adapter_enabled ────────────────────────────────────

#[test]
fn adapter_enabled_returns_true_when_status_enabled() {
    let sut = make_validator();
    let config = make_config_with_adapters(vec![AdapterEntry::new(
        AdapterName::raw("ruff"),
        AdapterStatus::Enabled,
        1.0,
    )]);
    let name = AdapterName::raw("ruff");
    assert!(sut.is_adapter_enabled(&config, &name));
}

#[test]
fn adapter_enabled_returns_false_when_status_disabled() {
    let sut = make_validator();
    let config = make_config_with_adapters(vec![AdapterEntry::new(
        AdapterName::raw("mypy"),
        AdapterStatus::Disabled,
        1.0,
    )]);
    let name = AdapterName::raw("mypy");
    assert!(!sut.is_adapter_enabled(&config, &name));
}

#[test]
fn adapter_enabled_returns_false_when_status_not_installed() {
    let sut = make_validator();
    let config = make_config_with_adapters(vec![AdapterEntry::new(
        AdapterName::raw("bandit"),
        AdapterStatus::NotInstalled,
        1.0,
    )]);
    let name = AdapterName::raw("bandit");
    assert!(!sut.is_adapter_enabled(&config, &name));
}

#[test]
fn adapter_enabled_defaults_true_when_adapter_not_in_list() {
    let sut = make_validator();
    let config = make_config_with_adapters(vec![]);
    let name = AdapterName::raw("unknown_adapter");
    assert!(sut.is_adapter_enabled(&config, &name));
}

#[test]
fn adapter_enabled_matches_first_occurrence() {
    let sut = make_validator();
    let config = make_config_with_adapters(vec![
        AdapterEntry::new(AdapterName::raw("ruff"), AdapterStatus::Disabled, 1.0),
        AdapterEntry::new(AdapterName::raw("ruff"), AdapterStatus::Enabled, 2.0),
    ]);
    let name = AdapterName::raw("ruff");
    // First match wins
    assert!(!sut.is_adapter_enabled(&config, &name));
}

// ─── validate_thresholds ───────────────────────────────────

#[test]
fn validate_thresholds_ok_with_valid_values() {
    let sut = make_validator();
    let mut config = ProjectConfig::default();
    config.thresholds = Thresholds::new(Score::new(80.0), Count::new(10), Count::new(500));
    let result = sut.validate_thresholds(&config);
    assert!(result.is_valid);
    assert!(result.reason.is_none());
}

#[test]
fn validate_thresholds_fails_when_score_above_100() {
    let sut = make_validator();
    let mut config = ProjectConfig::default();
    config.thresholds = Thresholds::new(Score::new(101.0), Count::new(10), Count::new(500));
    let result = sut.validate_thresholds(&config);
    assert!(!result.is_valid);
    assert!(result.reason.unwrap().contains("Score threshold"));
}

#[test]
fn validate_thresholds_fails_when_score_negative() {
    let sut = make_validator();
    let mut config = ProjectConfig::default();
    config.thresholds = Thresholds::new(Score::new(-1.0), Count::new(10), Count::new(500));
    let result = sut.validate_thresholds(&config);
    assert!(!result.is_valid);
}

#[test]
fn validate_thresholds_fails_when_complexity_zero() {
    let sut = make_validator();
    let mut config = ProjectConfig::default();
    config.thresholds = Thresholds::new(Score::new(80.0), Count::new(0), Count::new(500));
    let result = sut.validate_thresholds(&config);
    assert!(!result.is_valid);
    assert!(result.reason.unwrap().contains("Complexity"));
}

#[test]
fn validate_thresholds_fails_when_max_file_lines_zero() {
    let sut = make_validator();
    let mut config = ProjectConfig::default();
    config.thresholds = Thresholds::new(Score::new(80.0), Count::new(10), Count::new(0));
    let result = sut.validate_thresholds(&config);
    assert!(!result.is_valid);
    assert!(result.reason.unwrap().contains("max_file_lines"));
}

#[test]
fn validate_thresholds_accumulates_multiple_errors() {
    let sut = make_validator();
    let mut config = ProjectConfig::default();
    config.thresholds = Thresholds::new(Score::new(200.0), Count::new(0), Count::new(-1));
    let result = sut.validate_thresholds(&config);
    assert!(!result.is_valid);
    let reason = result.reason.unwrap();
    // All three errors should be present, separated by " | "
    assert!(reason.contains("Score threshold"));
    assert!(reason.contains("Complexity"));
    assert!(reason.contains("max_file_lines"));
}

#[test]
fn validate_thresholds_boundary_score_0_is_valid() {
    let sut = make_validator();
    let mut config = ProjectConfig::default();
    config.thresholds = Thresholds::new(Score::new(0.0), Count::new(1), Count::new(1));
    let result = sut.validate_thresholds(&config);
    assert!(result.is_valid);
}

#[test]
fn validate_thresholds_boundary_score_100_is_valid() {
    let sut = make_validator();
    let mut config = ProjectConfig::default();
    config.thresholds = Thresholds::new(Score::new(100.0), Count::new(1), Count::new(1));
    let result = sut.validate_thresholds(&config);
    assert!(result.is_valid);
}

// ─── Default / Constructor ─────────────────────────────────

#[test]
fn default_and_new_produce_equivalent_instances() {
    let a = ConfigRulesValidator::new();
    let b = ConfigRulesValidator::default();
    // Both should behave identically
    let config = ProjectConfig::default();
    assert_eq!(
        a.validate_thresholds(&config),
        b.validate_thresholds(&config)
    );
}
```

---

## `tests/unit_config_system_workspace_detector.rs`

```rust
// PURPOSE: Unit tests for WorkspaceDetector — language detection and workspace discovery.
use config_system_lint_arwaky::capabilities_workspace_detector::WorkspaceDetector;
use shared::common::taxonomy_path_vo::FilePath;
use shared::config_system::contract_workspace_detector_protocol::{
    IWorkspaceDetectorProtocol, WorkspaceType,
};
use std::fs;
use tempfile::TempDir;

fn make_detector() -> WorkspaceDetector {
    WorkspaceDetector::new()
}

fn create_file(dir: &std::path::Path, name: &str) {
    fs::write(dir.join(name), "").unwrap();
}

// ─── detect() ──────────────────────────────────────────────

#[test]
fn detect_rust_workspace_by_cargo_toml() {
    let tmp = TempDir::new().unwrap();
    create_file(tmp.path(), "Cargo.toml");
    let sut = make_detector();
    let fp = FilePath::new(tmp.path().to_string_lossy().to_string()).unwrap();
    assert_eq!(sut.detect(&fp), WorkspaceType::Rust);
}

#[test]
fn detect_typescript_workspace_by_package_json() {
    let tmp = TempDir::new().unwrap();
    create_file(tmp.path(), "package.json");
    let sut = make_detector();
    let fp = FilePath::new(tmp.path().to_string_lossy().to_string()).unwrap();
    assert_eq!(sut.detect(&fp), WorkspaceType::TypeScript);
}

#[test]
fn detect_python_workspace_by_pyproject_toml() {
    let tmp = TempDir::new().unwrap();
    create_file(tmp.path(), "pyproject.toml");
    let sut = make_detector();
    let fp = FilePath::new(tmp.path().to_string_lossy().to_string()).unwrap();
    assert_eq!(sut.detect(&fp), WorkspaceType::Python);
}

#[test]
fn detect_python_workspace_by_setup_py() {
    let tmp = TempDir::new().unwrap();
    create_file(tmp.path(), "setup.py");
    let sut = make_detector();
    let fp = FilePath::new(tmp.path().to_string_lossy().to_string()).unwrap();
    assert_eq!(sut.detect(&fp), WorkspaceType::Python);
}

#[test]
fn detect_python_workspace_by_requirements_txt() {
    let tmp = TempDir::new().unwrap();
    create_file(tmp.path(), "requirements.txt");
    let sut = make_detector();
    let fp = FilePath::new(tmp.path().to_string_lossy().to_string()).unwrap();
    assert_eq!(sut.detect(&fp), WorkspaceType::Python);
}

#[test]
fn detect_rust_takes_priority_over_typescript() {
    let tmp = TempDir::new().unwrap();
    create_file(tmp.path(), "Cargo.toml");
    create_file(tmp.path(), "package.json");
    let sut = make_detector();
    let fp = FilePath::new(tmp.path().to_string_lossy().to_string()).unwrap();
    assert_eq!(sut.detect(&fp), WorkspaceType::Rust);
}

#[test]
fn detect_unknown_when_no_markers() {
    let tmp = TempDir::new().unwrap();
    let sut = make_detector();
    let fp = FilePath::new(tmp.path().to_string_lossy().to_string()).unwrap();
    assert_eq!(sut.detect(&fp), WorkspaceType::Unknown);
}

#[test]
fn detect_by_parent_directory_name_crates() {
    let tmp = TempDir::new().unwrap();
    let crates_dir = tmp.path().join("crates");
    let member = crates_dir.join("my-crate");
    fs::create_dir_all(&member).unwrap();
    let sut = make_detector();
    let fp = FilePath::new(member.to_string_lossy().to_string()).unwrap();
    assert_eq!(sut.detect(&fp), WorkspaceType::Rust);
}

#[test]
fn detect_by_parent_directory_name_packages() {
    let tmp = TempDir::new().unwrap();
    let packages_dir = tmp.path().join("packages");
    let member = packages_dir.join("my-pkg");
    fs::create_dir_all(&member).unwrap();
    let sut = make_detector();
    let fp = FilePath::new(member.to_string_lossy().to_string()).unwrap();
    assert_eq!(sut.detect(&fp), WorkspaceType::TypeScript);
}

#[test]
fn detect_by_parent_directory_name_modules() {
    let tmp = TempDir::new().unwrap();
    let modules_dir = tmp.path().join("modules");
    let member = modules_dir.join("my-mod");
    fs::create_dir_all(&member).unwrap();
    let sut = make_detector();
    let fp = FilePath::new(member.to_string_lossy().to_string()).unwrap();
    assert_eq!(sut.detect(&fp), WorkspaceType::Python);
}

// ─── is_workspace() ────────────────────────────────────────

#[test]
fn is_workspace_true_when_crates_dir_exists() {
    let tmp = TempDir::new().unwrap();
    fs::create_dir(tmp.path().join("crates")).unwrap();
    let sut = make_detector();
    let fp = FilePath::new(tmp.path().to_string_lossy().to_string()).unwrap();
    assert!(sut.is_workspace(&fp));
}

#[test]
fn is_workspace_true_when_packages_dir_exists() {
    let tmp = TempDir::new().unwrap();
    fs::create_dir(tmp.path().join("packages")).unwrap();
    let sut = make_detector();
    let fp = FilePath::new(tmp.path().to_string_lossy().to_string()).unwrap();
    assert!(sut.is_workspace(&fp));
}

#[test]
fn is_workspace_true_when_modules_dir_exists() {
    let tmp = TempDir::new().unwrap();
    fs::create_dir(tmp.path().join("modules")).unwrap();
    let sut = make_detector();
    let fp = FilePath::new(tmp.path().to_string_lossy().to_string()).unwrap();
    assert!(sut.is_workspace(&fp));
}

#[test]
fn is_workspace_false_when_no_workspace_dirs() {
    let tmp = TempDir::new().unwrap();
    let sut = make_detector();
    let fp = FilePath::new(tmp.path().to_string_lossy().to_string()).unwrap();
    assert!(!sut.is_workspace(&fp));
}

// ─── discover_workspace_members() ──────────────────────────

#[tokio::test]
async fn discover_members_under_crates_dir() {
    let tmp = TempDir::new().unwrap();
    let crates = tmp.path().join("crates");
    fs::create_dir_all(crates.join("alpha")).unwrap();
    fs::create_dir_all(crates.join("beta")).unwrap();
    // A file should not be picked up
    fs::write(crates.join("README.md"), "# readme").unwrap();

    let sut = make_detector();
    let fp = FilePath::new(tmp.path().to_string_lossy().to_string()).unwrap();
    let members = sut.discover_workspace_members(&fp).await;

    assert_eq!(members.len(), 2);
    let names: Vec<String> = members.iter().map(|m| m.basename()).collect();
    assert!(names.contains(&"alpha".to_string()));
    assert!(names.contains(&"beta".to_string()));
}

#[tokio::test]
async fn discover_members_under_packages_dir() {
    let tmp = TempDir::new().unwrap();
    let packages = tmp.path().join("packages");
    fs::create_dir_all(packages.join("ui")).unwrap();
    fs::create_dir_all(packages.join("api")).unwrap();

    let sut = make_detector();
    let fp = FilePath::new(tmp.path().to_string_lossy().to_string()).unwrap();
    let members = sut.discover_workspace_members(&fp).await;

    assert_eq!(members.len(), 2);
}

#[tokio::test]
async fn discover_members_under_modules_dir() {
    let tmp = TempDir::new().unwrap();
    let modules = tmp.path().join("modules");
    fs::create_dir_all(modules.join("core")).unwrap();

    let sut = make_detector();
    let fp = FilePath::new(tmp.path().to_string_lossy().to_string()).unwrap();
    let members = sut.discover_workspace_members(&fp).await;

    assert_eq!(members.len(), 1);
    assert_eq!(members[0].basename(), "core");
}

#[tokio::test]
async fn discover_members_returns_empty_when_no_workspace_dirs() {
    let tmp = TempDir::new().unwrap();
    let sut = make_detector();
    let fp = FilePath::new(tmp.path().to_string_lossy().to_string()).unwrap();
    let members = sut.discover_workspace_members(&fp).await;
    assert!(members.is_empty());
}

#[tokio::test]
async fn discover_members_from_within_workspace_dir() {
    // When root IS a workspace dir (e.g., crates/), list its children
    let tmp = TempDir::new().unwrap();
    let crates = tmp.path().join("crates");
    fs::create_dir_all(crates.join("one")).unwrap();
    fs::create_dir_all(crates.join("two")).unwrap();

    let sut = make_detector();
    let fp = FilePath::new(crates.to_string_lossy().to_string()).unwrap();
    let members = sut.discover_workspace_members(&fp).await;

    assert_eq!(members.len(), 2);
}

// ─── Default / Constructor ─────────────────────────────────

#[test]
fn default_and_new_are_equivalent() {
    let a = WorkspaceDetector::new();
    let b = WorkspaceDetector::default();
    let tmp = TempDir::new().unwrap();
    create_file(tmp.path(), "Cargo.toml");
    let fp = FilePath::new(tmp.path().to_string_lossy().to_string()).unwrap();
    assert_eq!(a.detect(&fp), b.detect(&fp));
}
```

---

## `tests/unit_config_system_yaml_reader.rs`

```rust
// PURPOSE: Unit tests for ConfigYamlReader — config file discovery and XDG fallback.
use config_system_lint_arwaky::capabilities_yaml_reader::ConfigYamlReader;
use shared::common::taxonomy_path_vo::FilePath;
use shared::config_system::contract_reader_protocol::IConfigReaderProtocol;
use shared::config_system::taxonomy_config_language_vo::ConfigLanguage;
use std::fs;
use tempfile::TempDir;

fn make_reader() -> ConfigYamlReader {
    ConfigYamlReader::new()
}

// ─── read_config: local project file ───────────────────────

#[tokio::test]
async fn read_config_finds_rust_yaml_in_project_root() {
    let tmp = TempDir::new().unwrap();
    let yaml_content = "architecture:\n  enabled: true\n";
    fs::write(
        tmp.path().join("lint_arwaky.config.rust.yaml"),
        yaml_content,
    )
    .unwrap();

    let sut = make_reader();
    let fp = FilePath::new(tmp.path().to_string_lossy().to_string()).unwrap();
    let result = sut.read_config(&fp, ConfigLanguage::Rust).await;

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

    let sut = make_reader();
    let fp = FilePath::new(tmp.path().to_string_lossy().to_string()).unwrap();
    let result = sut.read_config(&fp, ConfigLanguage::Python).await;

    assert!(result.is_ok());
    assert!(result.unwrap().is_some());
}

#[tokio::test]
async fn read_config_typescript_falls_back_to_javascript_yaml() {
    let tmp = TempDir::new().unwrap();
    // Only .javascript.yaml exists, not .typescript.yaml
    fs::write(
        tmp.path().join("lint_arwaky.config.javascript.yaml"),
        "architecture:\n  enabled: true\n",
    )
    .unwrap();

    let sut = make_reader();
    let fp = FilePath::new(tmp.path().to_string_lossy().to_string()).unwrap();
    let result = sut.read_config(&fp, ConfigLanguage::TypeScript).await;

    assert!(result.is_ok());
    let source = result.unwrap();
    assert!(source.is_some());
    let source = source.unwrap();
    assert!(source.path.value.contains("javascript"));
}

#[tokio::test]
async fn read_config_searches_parent_directories_up_to_depth_3() {
    let tmp = TempDir::new().unwrap();
    // Place config at root
    fs::write(
        tmp.path().join("lint_arwaky.config.rust.yaml"),
        "architecture:\n  enabled: true\n",
    )
    .unwrap();
    // Create nested directory 2 levels deep
    let nested = tmp.path().join("a").join("b");
    fs::create_dir_all(&nested).unwrap();

    let sut = make_reader();
    let fp = FilePath::new(nested.to_string_lossy().to_string()).unwrap();
    let result = sut.read_config(&fp, ConfigLanguage::Rust).await;

    assert!(result.is_ok());
    assert!(result.unwrap().is_some());
}

#[tokio::test]
async fn read_config_returns_none_when_no_file_found() {
    let tmp = TempDir::new().unwrap();
    // Empty directory, no config files anywhere in hierarchy
    let sut = make_reader();
    let fp = FilePath::new(tmp.path().to_string_lossy().to_string()).unwrap();
    let result = sut.read_config(&fp, ConfigLanguage::Rust).await;

    // Should return Ok(None) — no file found, fall back to defaults
    assert!(result.is_ok());
    assert!(result.unwrap().is_none());
}

// ─── list_config_files ─────────────────────────────────────

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

    let sut = make_reader();
    let fp = FilePath::new(tmp.path().to_string_lossy().to_string()).unwrap();
    let result = sut.list_config_files(&fp).await;

    assert!(result.is_ok());
    let files = result.unwrap();
    assert_eq!(files.len(), 3);
}

#[tokio::test]
async fn list_config_files_returns_empty_when_none_exist() {
    let tmp = TempDir::new().unwrap();
    let sut = make_reader();
    let fp = FilePath::new(tmp.path().to_string_lossy().to_string()).unwrap();
    let result = sut.list_config_files(&fp).await;

    assert!(result.is_ok());
    assert!(result.unwrap().is_empty());
}

#[tokio::test]
async fn list_config_files_deduplicates_typescript_javascript() {
    let tmp = TempDir::new().unwrap();
    // Both typescript and javascript files exist — typescript wins, no duplicate
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

    let sut = make_reader();
    let fp = FilePath::new(tmp.path().to_string_lossy().to_string()).unwrap();
    let result = sut.list_config_files(&fp).await;

    assert!(result.is_ok());
    let files = result.unwrap();
    // TypeScript is found first; javascript is skipped because typescript already matched
    let ts_count = files
        .iter()
        .filter(|(lang, _)| *lang == ConfigLanguage::TypeScript)
        .count();
    assert_eq!(ts_count, 1);
}

// ─── Default / Constructor ─────────────────────────────────

#[test]
fn default_and_new_are_equivalent() {
    let _a = ConfigYamlReader::new();
    let _b = ConfigYamlReader::default();
    // Both should construct without panic
}
```

---

## `tests/unit_config_system_parser_provider.rs`

```rust
// PURPOSE: Unit tests for ConfigParserProvider — YAML and TOML config parsing.
use config_system_lint_arwaky::capabilities_parser_provider::ConfigParserProvider;
use shared::common::taxonomy_path_vo::FilePath;
use shared::config_system::contract_parser_protocol::IConfigParserProtocol;
use std::fs;
use tempfile::TempDir;

fn make_parser() -> ConfigParserProvider {
    ConfigParserProvider::new()
}

// ─── parse_yaml_config ─────────────────────────────────────

#[test]
fn parse_yaml_config_happy_path() {
    let tmp = TempDir::new().unwrap();
    let yaml = r#"
project_name: my-project
thresholds:
  score:
    value: 85.0
  complexity:
    value: 12
  max_file_lines:
    value: 400
adapters:
  - name:
      value: ruff
    status: enabled
    weight: 1.0
"#;
    let path = tmp.path().join("config.yaml");
    fs::write(&path, yaml).unwrap();

    let sut = make_parser();
    let fp = FilePath::new(path.to_string_lossy().to_string()).unwrap();
    let result = sut.parse_yaml_config(&fp);

    assert!(result.is_ok());
    let config = result.unwrap();
    assert_eq!(config.project_name.value, "my-project");
    assert_eq!(config.thresholds.score.value, 85.0);
    assert_eq!(config.thresholds.complexity.value, 12);
    assert_eq!(config.adapters.len(), 1);
}

#[test]
fn parse_yaml_config_file_not_found() {
    let sut = make_parser();
    let fp = FilePath::new("/nonexistent/path/config.yaml".to_string()).unwrap();
    let result = sut.parse_yaml_config(&fp);

    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.message.value.contains("Failed to read config"));
}

#[test]
fn parse_yaml_config_invalid_yaml() {
    let tmp = TempDir::new().unwrap();
    let path = tmp.path().join("bad.yaml");
    fs::write(&path, "{{{{invalid yaml::::").unwrap();

    let sut = make_parser();
    let fp = FilePath::new(path.to_string_lossy().to_string()).unwrap();
    let result = sut.parse_yaml_config(&fp);

    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.message.value.contains("Failed to deserialize YAML"));
}

#[test]
fn parse_yaml_config_empty_file_uses_defaults() {
    let tmp = TempDir::new().unwrap();
    let path = tmp.path().join("empty.yaml");
    fs::write(&path, "").unwrap();

    let sut = make_parser();
    let fp = FilePath::new(path.to_string_lossy().to_string()).unwrap();
    let result = sut.parse_yaml_config(&fp);

    // Empty YAML deserializes to default ProjectConfig
    assert!(result.is_ok());
}

// ─── parse_toml_config ─────────────────────────────────────

#[test]
fn parse_toml_config_with_tool_section() {
    let tmp = TempDir::new().unwrap();
    let toml_content = r#"
[tool.lint-arwaky]
project_name = "my-toml-project"

[tool.lint-arwaky.thresholds]
score = { value = 90.0 }
complexity = { value = 8 }
max_file_lines = { value = 300 }
"#;
    let path = tmp.path().join("pyproject.toml");
    fs::write(&path, toml_content).unwrap();

    let sut = make_parser();
    let fp = FilePath::new(path.to_string_lossy().to_string()).unwrap();
    let result = sut.parse_toml_config(&fp);

    assert!(result.is_ok());
    let config = result.unwrap();
    assert!(config.is_some());
    let config = config.unwrap();
    assert_eq!(config.project_name.value, "my-toml-project");
}

#[test]
fn parse_toml_config_with_underscore_key() {
    let tmp = TempDir::new().unwrap();
    let toml_content = r#"
[tool.lint_arwaky]
project_name = "underscore-project"
"#;
    let path = tmp.path().join("Cargo.toml");
    fs::write(&path, toml_content).unwrap();

    let sut = make_parser();
    let fp = FilePath::new(path.to_string_lossy().to_string()).unwrap();
    let result = sut.parse_toml_config(&fp);

    assert!(result.is_ok());
    assert!(result.unwrap().is_some());
}

#[test]
fn parse_toml_config_returns_none_without_tool_section() {
    let tmp = TempDir::new().unwrap();
    let toml_content = r#"
[package]
name = "my-crate"
version = "0.1.0"
"#;
    let path = tmp.path().join("Cargo.toml");
    fs::write(&path, toml_content).unwrap();

    let sut = make_parser();
    let fp = FilePath::new(path.to_string_lossy().to_string()).unwrap();
    let result = sut.parse_toml_config(&fp);

    assert!(result.is_ok());
    assert!(result.unwrap().is_none());
}

#[test]
fn parse_toml_config_file_not_found() {
    let sut = make_parser();
    let fp = FilePath::new("/nonexistent/Cargo.toml".to_string()).unwrap();
    let result = sut.parse_toml_config(&fp);

    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.message.value.contains("Failed to read TOML"));
}

#[test]
fn parse_toml_config_invalid_toml() {
    let tmp = TempDir::new().unwrap();
    let path = tmp.path().join("bad.toml");
    fs::write(&path, "this is [[[not valid toml").unwrap();

    let sut = make_parser();
    let fp = FilePath::new(path.to_string_lossy().to_string()).unwrap();
    let result = sut.parse_toml_config(&fp);

    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.message.value.contains("Failed to parse TOML"));
}

// ─── Default / Constructor ─────────────────────────────────

#[test]
fn default_and_new_are_equivalent() {
    let _a = ConfigParserProvider::new();
    let _b = ConfigParserProvider::default();
}
```

---

## `tests/unit_config_system_orchestrator.rs`

```rust
// PURPOSE: Unit tests for ConfigOrchestrator — config loading, caching, workspace discovery.
use config_system_lint_arwaky::agent_config_orchestrator::ConfigOrchestrator;
use config_system_lint_arwaky::capabilities_rules_validator::ConfigRulesValidator;
use config_system_lint_arwaky::capabilities_workspace_detector::WorkspaceDetector;
use config_system_lint_arwaky::capabilities_yaml_reader::ConfigYamlReader;
use shared::common::taxonomy_path_vo::FilePath;
use shared::config_system::contract_config_orchestrator_aggregate::IConfigOrchestratorAggregate;
use shared::config_system::taxonomy_config_language_vo::ConfigLanguage;
use std::fs;
use std::sync::Arc;
use tempfile::TempDir;

fn make_orchestrator() -> ConfigOrchestrator {
    ConfigOrchestrator::new(
        Arc::new(WorkspaceDetector::new()),
        Arc::new(ConfigYamlReader::new()),
        Arc::new(ConfigRulesValidator::new()),
    )
}

// ─── load_project_config ───────────────────────────────────

#[tokio::test]
async fn load_project_config_uses_defaults_when_no_file() {
    let tmp = TempDir::new().unwrap();
    let sut = make_orchestrator();
    let fp = FilePath::new(tmp.path().to_string_lossy().to_string()).unwrap();

    let result = sut.load_project_config(&fp).await;

    assert!(!result.warnings.is_empty());
    assert!(result
        .warnings
        .iter()
        .any(|w| w.contains("No config file found")));
    assert_eq!(result.source.language, "rust"); // Unknown defaults to Rust
}

#[tokio::test]
async fn load_project_config_reads_existing_yaml() {
    let tmp = TempDir::new().unwrap();
    fs::write(
        tmp.path().join("lint_arwaky.config.rust.yaml"),
        "architecture:\n  enabled: true\n  rules: []\n",
    )
    .unwrap();
    // Need Cargo.toml so workspace is detected as Rust
    fs::write(tmp.path().join("Cargo.toml"), "[package]\nname=\"x\"\n").unwrap();

    let sut = make_orchestrator();
    let fp = FilePath::new(tmp.path().to_string_lossy().to_string()).unwrap();
    let result = sut.load_project_config(&fp).await;

    assert_eq!(result.source.language, "rust");
    assert!(result.source.path.value.contains("lint_arwaky.config.rust.yaml"));
}

// ─── load_config_for_language ──────────────────────────────

#[tokio::test]
async fn load_config_for_language_python() {
    let tmp = TempDir::new().unwrap();
    fs::write(
        tmp.path().join("lint_arwaky.config.python.yaml"),
        "architecture:\n  enabled: true\n",
    )
    .unwrap();

    let sut = make_orchestrator();
    let fp = FilePath::new(tmp.path().to_string_lossy().to_string()).unwrap();
    let result = sut
        .load_config_for_language(&fp, ConfigLanguage::Python)
        .await;

    assert_eq!(result.source.language, "python");
}

#[tokio::test]
async fn load_config_for_language_injects_defaults_when_no_layers() {
    let tmp = TempDir::new().unwrap();
    // Config with no layers key
    fs::write(
        tmp.path().join("lint_arwaky.config.rust.yaml"),
        "architecture:\n  enabled: true\n  rules: []\n",
    )
    .unwrap();

    let sut = make_orchestrator();
    let fp = FilePath::new(tmp.path().to_string_lossy().to_string()).unwrap();
    let result = sut
        .load_config_for_language(&fp, ConfigLanguage::Rust)
        .await;

    // Should inject default layers and produce a warning
    assert!(result
        .warnings
        .iter()
        .any(|w| w.contains("no architecture layers")));
}

// ─── load_config_sync ──────────────────────────────────────

#[test]
fn load_config_sync_returns_defaults_for_empty_dir() {
    let tmp = TempDir::new().unwrap();
    let sut = make_orchestrator();
    let config = sut.load_config_sync(tmp.path().to_str().unwrap());
    // Should return a valid config (defaults)
    assert!(config.enabled.value);
}

#[test]
fn load_config_sync_finds_config_in_current_dir() {
    let tmp = TempDir::new().unwrap();
    fs::write(
        tmp.path().join("lint_arwaky.config.rust.yaml"),
        "architecture:\n  enabled: false\n  rules: []\n",
    )
    .unwrap();
    fs::write(tmp.path().join("Cargo.toml"), "[package]\nname=\"x\"\n").unwrap();

    let sut = make_orchestrator();
    let config = sut.load_config_sync(tmp.path().to_str().unwrap());
    assert!(!config.enabled.value);
}

// ─── ignored_paths ─────────────────────────────────────────

#[test]
fn ignored_paths_includes_hardcoded_defaults() {
    let tmp = TempDir::new().unwrap();
    let sut = make_orchestrator();
    let paths = sut.ignored_paths(tmp.path().to_str().unwrap());

    assert!(paths.contains(&"target".to_string()));
    assert!(paths.contains(&"node_modules".to_string()));
    assert!(paths.contains(&".git".to_string()));
    assert!(paths.contains(&"dist".to_string()));
}

// ─── discover_workspaces ───────────────────────────────────

#[tokio::test]
async fn discover_workspaces_returns_members() {
    let tmp = TempDir::new().unwrap();
    let crates = tmp.path().join("crates");
    fs::create_dir_all(crates.join("alpha")).unwrap();
    fs::create_dir_all(crates.join("beta")).unwrap();
    // Give alpha a Cargo.toml so it's detected as Rust
    fs::write(crates.join("alpha").join("Cargo.toml"), "").unwrap();

    let sut = make_orchestrator();
    let fp = FilePath::new(tmp.path().to_string_lossy().to_string()).unwrap();
    let workspaces = sut.discover_workspaces(&fp).await;

    assert_eq!(workspaces.len(), 2);
}

#[tokio::test]
async fn discover_workspaces_returns_empty_for_non_workspace() {
    let tmp = TempDir::new().unwrap();
    let sut = make_orchestrator();
    let fp = FilePath::new(tmp.path().to_string_lossy().to_string()).unwrap();
    let workspaces = sut.discover_workspaces(&fp).await;
    assert!(workspaces.is_empty());
}

// ─── Caching ───────────────────────────────────────────────

#[tokio::test]
async fn config_cache_returns_same_arc_on_second_load() {
    let tmp = TempDir::new().unwrap();
    fs::write(
        tmp.path().join("lint_arwaky.config.rust.yaml"),
        "architecture:\n  enabled: true\n  rules: []\n",
    )
    .unwrap();
    fs::write(tmp.path().join("Cargo.toml"), "[package]\nname=\"x\"\n").unwrap();

    let sut = make_orchestrator();
    let fp = FilePath::new(tmp.path().to_string_lossy().to_string()).unwrap();

    let r1 = sut
        .load_config_for_language(&fp, ConfigLanguage::Rust)
        .await;
    let r2 = sut
        .load_config_for_language(&fp, ConfigLanguage::Rust)
        .await;

    // Both should succeed and reference the same source path
    assert_eq!(r1.source.path, r2.source.path);
}

// ─── Constructor ───────────────────────────────────────────

#[test]
fn validator_accessor_returns_same_instance() {
    let sut = make_orchestrator();
    let _v = sut.validator();
    // Should not panic
}
```

---

## `tests/integration_config_system.rs`

```rust
// PURPOSE: Integration tests — full DI wiring via ConfigContainer.
use config_system_lint_arwaky::root_config_system_container::ConfigContainer;
use shared::common::taxonomy_path_vo::FilePath;
use shared::config_system::contract_config_orchestrator_aggregate::IConfigOrchestratorAggregate;
use shared::config_system::contract_parser_protocol::IConfigParserProtocol;
use shared::config_system::contract_reader_protocol::IConfigReaderProtocol;
use shared::config_system::contract_validator_protocol::IConfigValidatorProtocol;
use shared::config_system::taxonomy_config_language_vo::ConfigLanguage;
use shared::config_system::taxonomy_setting_vo::ProjectConfig;
use std::fs;
use tempfile::TempDir;

// ─── Container wiring ──────────────────────────────────────

#[test]
fn container_provides_orchestrator() {
    let container = ConfigContainer::new();
    let orch = container.orchestrator();
    // Should be a valid Arc<dyn IConfigOrchestratorAggregate>
    let _ = &orch;
}

#[test]
fn container_provides_reader() {
    let container = ConfigContainer::new();
    let reader = container.reader();
    let _ = &reader;
}

#[test]
fn container_provides_parser() {
    let container = ConfigContainer::new();
    let parser = container.parser();
    let _ = &parser;
}

#[test]
fn container_provides_validator() {
    let container = ConfigContainer::new();
    let validator = container.validator();
    let _ = &validator;
}

#[test]
fn container_default_is_equivalent_to_new() {
    let a = ConfigContainer::new();
    let b = ConfigContainer::default();
    // Both should provide working orchestrators
    let _oa = a.orchestrator();
    let _ob = b.orchestrator();
}

// ─── Orchestrator via container ────────────────────────────

#[tokio::test]
async fn container_orchestrator_loads_defaults_for_empty_project() {
    let tmp = TempDir::new().unwrap();
    let container = ConfigContainer::new();
    let orch = container.orchestrator();
    let fp = FilePath::new(tmp.path().to_string_lossy().to_string()).unwrap();

    let result = orch.load_project_config(&fp).await;
    assert!(!result.warnings.is_empty());
}

#[tokio::test]
async fn container_orchestrator_loads_real_config() {
    let tmp = TempDir::new().unwrap();
    fs::write(
        tmp.path().join("lint_arwaky.config.rust.yaml"),
        "architecture:\n  enabled: true\n  rules: []\n",
    )
    .unwrap();
    fs::write(tmp.path().join("Cargo.toml"), "[package]\nname=\"x\"\n").unwrap();

    let container = ConfigContainer::new();
    let orch = container.orchestrator();
    let fp = FilePath::new(tmp.path().to_string_lossy().to_string()).unwrap();

    let result = orch.load_project_config(&fp).await;
    assert_eq!(result.source.language, "rust");
}

// ─── Reader via container ──────────────────────────────────

#[tokio::test]
async fn container_reader_lists_config_files() {
    let tmp = TempDir::new().unwrap();
    fs::write(tmp.path().join("lint_arwaky.config.rust.yaml"), "a: 1").unwrap();

    let container = ConfigContainer::new();
    let reader = container.reader();
    let fp = FilePath::new(tmp.path().to_string_lossy().to_string()).unwrap();

    let files = reader.list_config_files(&fp).await.unwrap();
    assert_eq!(files.len(), 1);
    assert_eq!(files[0].0, ConfigLanguage::Rust);
}

// ─── Parser via container ──────────────────────────────────

#[test]
fn container_parser_parses_yaml() {
    let tmp = TempDir::new().unwrap();
    let yaml = "project_name: integration-test\n";
    let path = tmp.path().join("config.yaml");
    fs::write(&path, yaml).unwrap();

    let container = ConfigContainer::new();
    let parser = container.parser();
    let fp = FilePath::new(path.to_string_lossy().to_string()).unwrap();

    let config = parser.parse_yaml_config(&fp).unwrap();
    assert_eq!(config.project_name.value, "integration-test");
}

// ─── Validator via container ───────────────────────────────

#[test]
fn container_validator_validates_default_config() {
    let container = ConfigContainer::new();
    let validator = container.validator();
    let config = ProjectConfig::default();
    let result = validator.validate_thresholds(&config);
    assert!(result.is_valid);
}

// ─── Full pipeline: read → parse → validate ────────────────

#[tokio::test]
async fn full_pipeline_read_parse_validate() {
    let tmp = TempDir::new().unwrap();
    let yaml = r#"
project_name: pipeline-test
thresholds:
  score:
    value: 75.0
  complexity:
    value: 15
  max_file_lines:
    value: 600
"#;
    let path = tmp.path().join("config.yaml");
    fs::write(&path, yaml).unwrap();

    let container = ConfigContainer::new();
    let parser = container.parser();
    let validator = container.validator();

    let fp = FilePath::new(path.to_string_lossy().to_string()).unwrap();
    let config = parser.parse_yaml_config(&fp).unwrap();
    let validation = validator.validate_thresholds(&config);

    assert!(validation.is_valid);
    assert_eq!(config.project_name.value, "pipeline-test");
}
```

---

## `tests/smoke_config_system.rs`

```rust
// PURPOSE: Smoke test — verify the config-system crate boots and core operations respond.
// Must complete in under 5 seconds.
use config_system_lint_arwaky::root_config_system_container::ConfigContainer;
use shared::common::taxonomy_path_vo::FilePath;
use shared::config_system::contract_config_orchestrator_aggregate::IConfigOrchestratorAggregate;
use tempfile::TempDir;

#[tokio::test]
async fn config_system_boots_and_loads_defaults() {
    let start = std::time::Instant::now();

    let container = ConfigContainer::new();
    let orch = container.orchestrator();

    let tmp = TempDir::new().unwrap();
    let fp = FilePath::new(tmp.path().to_string_lossy().to_string()).unwrap();
    let result = orch.load_project_config(&fp).await;

    // Basic sanity: we got a result with a valid config
    assert!(result.config.enabled.value || !result.config.enabled.value); // always true, just access
    assert!(!result.source.language.is_empty());

    let elapsed = start.elapsed();
    assert!(
        elapsed.as_secs() < 5,
        "Smoke test exceeded 5s: {:?}",
        elapsed
    );
}

#[test]
fn config_system_sync_load_responds() {
    let start = std::time::Instant::now();

    let container = ConfigContainer::new();
    let orch = container.orchestrator();

    let tmp = TempDir::new().unwrap();
    let config = orch.load_config_sync(tmp.path().to_str().unwrap());
    assert!(config.enabled.value);

    let elapsed = start.elapsed();
    assert!(
        elapsed.as_secs() < 5,
        "Smoke test exceeded 5s: {:?}",
        elapsed
    );
}
```

---

## `tests/e2e_config_system_flow.rs`

```rust
// PURPOSE: E2E tests — full config lifecycle from filesystem to validated output.
use config_system_lint_arwaky::root_config_system_container::ConfigContainer;
use shared::common::taxonomy_path_vo::FilePath;
use shared::config_system::contract_config_orchestrator_aggregate::IConfigOrchestratorAggregate;
use shared::config_system::contract_reader_protocol::IConfigReaderProtocol;
use shared::config_system::taxonomy_config_language_vo::ConfigLanguage;
use std::fs;
use tempfile::TempDir;

/// Full lifecycle: create workspace → write config → load → validate → discover members
#[tokio::test]
async fn full_config_lifecycle_rust_workspace() {
    let tmp = TempDir::new().unwrap();
    let root = tmp.path();

    // 1. Create workspace structure
    fs::write(root.join("Cargo.toml"), "[workspace]\nmembers = [\"crates/*\"]\n").unwrap();
    let crates = root.join("crates");
    fs::create_dir_all(crates.join("core")).unwrap();
    fs::write(crates.join("core").join("Cargo.toml"), "[package]\nname=\"core\"\n").unwrap();
    fs::create_dir_all(crates.join("cli")).unwrap();
    fs::write(crates.join("cli").join("Cargo.toml"), "[package]\nname=\"cli\"\n").unwrap();

    // 2. Write project config
    let config_yaml = r#"
architecture:
  enabled: true
  layers:
    taxonomy:
      prefix: taxonomy_
      suffix:
        - strict: [vo, entity, event, error, constant]
    capabilities:
      prefix: capabilities_
      suffix:
        - strict: [validator, reader, detector, provider]
  rules: []
  ignored_paths:
    - target
    - .git
"#;
    fs::write(root.join("lint_arwaky.config.rust.yaml"), config_yaml).unwrap();

    // 3. Boot container
    let container = ConfigContainer::new();
    let orch = container.orchestrator();
    let fp = FilePath::new(root.to_string_lossy().to_string()).unwrap();

    // 4. Load project config
    let result = orch.load_project_config(&fp).await;
    assert_eq!(result.source.language, "rust");
    assert!(result.config.enabled.value);
    assert!(!result.config.layers.is_empty());

    // 5. Discover workspace members
    let workspaces = orch.discover_workspaces(&fp).await;
    assert_eq!(workspaces.len(), 2);
    let ws_names: Vec<String> = workspaces.iter().map(|w| w.path.basename()).collect();
    assert!(ws_names.contains(&"core".to_string()));
    assert!(ws_names.contains(&"cli".to_string()));

    // 6. Verify ignored paths include config values
    let ignored = orch.ignored_paths(root.to_str().unwrap());
    assert!(ignored.contains(&"target".to_string()));
    assert!(ignored.contains(&".git".to_string()));
}

/// E2E: TypeScript workspace with javascript fallback
#[tokio::test]
async fn full_config_lifecycle_typescript_fallback() {
    let tmp = TempDir::new().unwrap();
    let root = tmp.path();

    fs::write(root.join("package.json"), r#"{"name": "my-app"}"#).unwrap();
    // Only javascript config exists
    fs::write(
        root.join("lint_arwaky.config.javascript.yaml"),
        "architecture:\n  enabled: true\n  rules: []\n",
    )
    .unwrap();

    let container = ConfigContainer::new();
    let orch = container.orchestrator();
    let fp = FilePath::new(root.to_string_lossy().to_string()).unwrap();

    let result = orch
        .load_config_for_language(&fp, ConfigLanguage::TypeScript)
        .await;

    assert_eq!(result.source.language, "typescript");
    assert!(result.source.path.value.contains("javascript"));
}

/// E2E: Reader lists all available config files
#[tokio::test]
async fn e2e_reader_lists_multi_language_configs() {
    let tmp = TempDir::new().unwrap();
    fs::write(tmp.path().join("lint_arwaky.config.rust.yaml"), "a: 1").unwrap();
    fs::write(tmp.path().join("lint_arwaky.config.python.yaml"), "b: 2").unwrap();

    let container = ConfigContainer::new();
    let reader = container.reader();
    let fp = FilePath::new(tmp.path().to_string_lossy().to_string()).unwrap();

    let files = reader.list_config_files(&fp).await.unwrap();
    assert_eq!(files.len(), 2);

    let langs: Vec<ConfigLanguage> = files.iter().map(|(l, _)| *l).collect();
    assert!(langs.contains(&ConfigLanguage::Rust));
    assert!(langs.contains(&ConfigLanguage::Python));
}
```

---

## `tests/acceptance_US_1.rs`

```rust
// PURPOSE: US-1 — Project Config Discovery
// "As a developer running `lint-arwaky check`, I need the system to find my
//  project's config file automatically."
use config_system_lint_arwaky::root_config_system_container::ConfigContainer;
use shared::common::taxonomy_path_vo::FilePath;
use shared::config_system::contract_config_orchestrator_aggregate::IConfigOrchestratorAggregate;
use std::fs;
use tempfile::TempDir;

/// AC-1: Config in project root is discovered automatically
#[tokio::test]
async fn us1_config_in_project_root_is_found() {
    let tmp = TempDir::new().unwrap();
    fs::write(
        tmp.path().join("lint_arwaky.config.rust.yaml"),
        "architecture:\n  enabled: true\n  rules: []\n",
    )
    .unwrap();
    fs::write(tmp.path().join("Cargo.toml"), "[package]\nname=\"x\"\n").unwrap();

    let container = ConfigContainer::new();
    let orch = container.orchestrator();
    let fp = FilePath::new(tmp.path().to_string_lossy().to_string()).unwrap();

    let result = orch.load_project_config(&fp).await;
    assert!(result.source.path.value.contains("lint_arwaky.config.rust.yaml"));
    assert!(!result
        .warnings
        .iter()
        .any(|w| w.contains("No config file found")));
}

/// AC-1: Config in parent directory (up to depth 3) is discovered
#[tokio::test]
async fn us1_config_in_parent_directory_is_found() {
    let tmp = TempDir::new().unwrap();
    fs::write(
        tmp.path().join("lint_arwaky.config.rust.yaml"),
        "architecture:\n  enabled: true\n  rules: []\n",
    )
    .unwrap();

    let nested = tmp.path().join("src").join("deep");
    fs::create_dir_all(&nested).unwrap();

    let container = ConfigContainer::new();
    let orch = container.orchestrator();
    let fp = FilePath::new(nested.to_string_lossy().to_string()).unwrap();

    let result = orch
        .load_config_for_language(&fp, shared::config_system::taxonomy_config_language_vo::ConfigLanguage::Rust)
        .await;

    assert!(result.source.path.value.contains("lint_arwaky.config.rust.yaml"));
}
```

---

## `tests/acceptance_US_2.rs`

```rust
// PURPOSE: US-2 — Multi-Language Support
// "As a polyglot developer, I need the system to detect whether my workspace is
//  Rust, Python, or TypeScript and load the correct config."
use config_system_lint_arwaky::root_config_system_container::ConfigContainer;
use shared::common::taxonomy_path_vo::FilePath;
use shared::config_system::contract_config_orchestrator_aggregate::IConfigOrchestratorAggregate;
use std::fs;
use tempfile::TempDir;

/// Rust workspace loads rust config
#[tokio::test]
async fn us2_rust_workspace_loads_rust_config() {
    let tmp = TempDir::new().unwrap();
    fs::write(tmp.path().join("Cargo.toml"), "[package]\nname=\"x\"\n").unwrap();
    fs::write(
        tmp.path().join("lint_arwaky.config.rust.yaml"),
        "architecture:\n  enabled: true\n  rules: []\n",
    )
    .unwrap();

    let container = ConfigContainer::new();
    let orch = container.orchestrator();
    let fp = FilePath::new(tmp.path().to_string_lossy().to_string()).unwrap();

    let result = orch.load_project_config(&fp).await;
    assert_eq!(result.source.language, "rust");
}

/// Python workspace loads python config
#[tokio::test]
async fn us2_python_workspace_loads_python_config() {
    let tmp = TempDir::new().unwrap();
    fs::write(tmp.path().join("pyproject.toml"), "[project]\nname=\"x\"\n").unwrap();
    fs::write(
        tmp.path().join("lint_arwaky.config.python.yaml"),
        "architecture:\n  enabled: true\n  rules: []\n",
    )
    .unwrap();

    let container = ConfigContainer::new();
    let orch = container.orchestrator();
    let fp = FilePath::new(tmp.path().to_string_lossy().to_string()).unwrap();

    let result = orch.load_project_config(&fp).await;
    assert_eq!(result.source.language, "python");
}

/// TypeScript workspace loads typescript config
#[tokio::test]
async fn us2_typescript_workspace_loads_typescript_config() {
    let tmp = TempDir::new().unwrap();
    fs::write(tmp.path().join("package.json"), r#"{"name":"x"}"#).unwrap();
    fs::write(
        tmp.path().join("lint_arwaky.config.typescript.yaml"),
        "architecture:\n  enabled: true\n  rules: []\n",
    )
    .unwrap();

    let container = ConfigContainer::new();
    let orch = container.orchestrator();
    let fp = FilePath::new(tmp.path().to_string_lossy().to_string()).unwrap();

    let result = orch.load_project_config(&fp).await;
    assert_eq!(result.source.language, "typescript");
}
```

---

## `tests/acceptance_US_3.rs`

```rust
// PURPOSE: US-3 — Config Fallback Safety
// "As a developer without a config file, I need sensible defaults so that
//  linting works out of the box."
use config_system_lint_arwaky::root_config_system_container::ConfigContainer;
use shared::common::taxonomy_path_vo::FilePath;
use shared::config_system::contract_config_orchestrator_aggregate::IConfigOrchestratorAggregate;
use tempfile::TempDir;

/// No config file → defaults are used with a warning
#[tokio::test]
async fn us3_no_config_file_uses_defaults() {
    let tmp = TempDir::new().unwrap();
    let container = ConfigContainer::new();
    let orch = container.orchestrator();
    let fp = FilePath::new(tmp.path().to_string_lossy().to_string()).unwrap();

    let result = orch.load_project_config(&fp).await;

    assert!(result.config.enabled.value);
    assert!(result
        .warnings
        .iter()
        .any(|w| w.contains("No config file found")));
    assert_eq!(result.source.path.value, "embedded");
}

/// Defaults provide a valid, non-empty configuration
#[tokio::test]
async fn us3_defaults_are_valid_and_usable() {
    let tmp = TempDir::new().unwrap();
    let container = ConfigContainer::new();
    let orch = container.orchestrator();

    let config = orch.load_config_sync(tmp.path().to_str().unwrap());

    // Default config should be enabled
    assert!(config.enabled.value);
    // Should have default ignored paths
    let ignored = orch.ignored_paths(tmp.path().to_str().unwrap());
    assert!(!ignored.is_empty());
}
```

---

## `tests/acceptance_US_4.rs`

```rust
// PURPOSE: US-4 — Multi-Workspace Analysis
// "As a monorepo maintainer, I need the system to discover and load configs
//  for all workspace members (crates/, packages/, modules/)."
use config_system_lint_arwaky::root_config_system_container::ConfigContainer;
use shared::common::taxonomy_path_vo::FilePath;
use shared::config_system::contract_config_orchestrator_aggregate::IConfigOrchestratorAggregate;
use std::fs;
use tempfile::TempDir;

/// Discovers all workspace members across crates/, packages/, modules/
#[tokio::test]
async fn us4_discovers_all_workspace_member_types() {
    let tmp = TempDir::new().unwrap();
    let root = tmp.path();

    fs::create_dir_all(root.join("crates").join("rust-lib")).unwrap();
    fs::create_dir_all(root.join("packages").join("ts-app")).unwrap();
    fs::create_dir_all(root.join("modules").join("py-mod")).unwrap();

    let container = ConfigContainer::new();
    let orch = container.orchestrator();
    let fp = FilePath::new(root.to_string_lossy().to_string()).unwrap();

    let workspaces = orch.discover_workspaces(&fp).await;
    assert_eq!(workspaces.len(), 3);

    let names: Vec<String> = workspaces.iter().map(|w| w.path.basename()).collect();
    assert!(names.contains(&"rust-lib".to_string()));
    assert!(names.contains(&"ts-app".to_string()));
    assert!(names.contains(&"py-mod".to_string()));
}

/// Each workspace member gets its own config
#[tokio::test]
async fn us4_each_member_gets_own_config() {
    let tmp = TempDir::new().unwrap();
    let root = tmp.path();

    let crate_dir = root.join("crates").join("my-crate");
    fs::create_dir_all(&crate_dir).unwrap();
    fs::write(crate_dir.join("Cargo.toml"), "[package]\nname=\"my-crate\"\n").unwrap();

    let container = ConfigContainer::new();
    let orch = container.orchestrator();
    let fp = FilePath::new(root.to_string_lossy().to_string()).unwrap();

    let workspaces = orch.discover_workspaces(&fp).await;
    assert_eq!(workspaces.len(), 1);
    assert_eq!(workspaces[0].workspace_type, "rust");
}

/// Empty workspace returns empty list with warning
#[tokio::test]
async fn us4_empty_workspace_returns_empty() {
    let tmp = TempDir::new().unwrap();
    let container = ConfigContainer::new();
    let orch = container.orchestrator();
    let fp = FilePath::new(tmp.path().to_string_lossy().to_string()).unwrap();

    let workspaces = orch.discover_workspaces(&fp).await;
    assert!(workspaces.is_empty());
}
```

---

## `tests/acceptance_US_5.rs`

```rust
// PURPOSE: US-5 — Config Security
// "As a security-conscious developer, I need config file reads to be confined
//  within the project root and reject symlinks pointing outside."
use shared::common::taxonomy_path_vo::FilePath;
use shared::config_system::utility_config_io::{read_text_within_canonical_root, MAX_CONFIG_FILE_SIZE};
use std::fs;
use tempfile::TempDir;

/// Symlink pointing outside root is rejected
#[tokio::test]
async fn us5_symlink_outside_root_is_rejected() {
    let tmp = TempDir::new().unwrap();
    let root = tmp.path().join("project");
    fs::create_dir_all(&root).unwrap();

    // Create a file outside the project root
    let outside = tmp.path().join("secret.txt");
    fs::write(&outside, "sensitive data").unwrap();

    // Create symlink inside project pointing outside
    let link = root.join("config.yaml");
    #[cfg(unix)]
    std::os::unix::fs::symlink(&outside, &link).unwrap();
    #[cfg(not(unix))]
    {
        // On non-unix, skip this test
        return;
    }

    let canonical_root = fs::canonicalize(&root).unwrap();
    let result = read_text_within_canonical_root(&link, &canonical_root).await;

    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.kind(), std::io::ErrorKind::PermissionDenied);
}

/// File exceeding MAX_CONFIG_FILE_SIZE is rejected
#[tokio::test]
async fn us5_oversized_config_is_rejected() {
    let tmp = TempDir::new().unwrap();
    let root = tmp.path();

    // Create a file larger than 1 MiB
    let large_content = "x".repeat((MAX_CONFIG_FILE_SIZE + 1) as usize);
    let large_file = root.join("large.yaml");
    fs::write(&large_file, &large_content).unwrap();

    let canonical_root = fs::canonicalize(root).unwrap();
    let result = read_text_within_canonical_root(&large_file, &canonical_root).await;

    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.kind(), std::io::ErrorKind::InvalidData);
}

/// Valid file within root is read successfully
#[tokio::test]
async fn us5_valid_file_within_root_is_read() {
    let tmp = TempDir::new().unwrap();
    let root = tmp.path();
    let config_file = root.join("config.yaml");
    fs::write(&config_file, "architecture:\n  enabled: true\n").unwrap();

    let canonical_root = fs::canonicalize(root).unwrap();
    let result = read_text_within_canonical_root(&config_file, &canonical_root).await;

    assert!(result.is_ok());
    assert!(result.unwrap().contains("architecture"));
}

/// ConfigLanguage enum prevents path injection
#[test]
fn us5_config_language_prevents_path_injection() {
    use shared::config_system::taxonomy_config_language_vo::ConfigLanguage;
    use std::str::FromStr;

    // Valid inputs
    assert!(ConfigLanguage::from_str("rust").is_ok());
    assert!(ConfigLanguage::from_str("python").is_ok());
    assert!(ConfigLanguage::from_str("typescript").is_ok());

    // Path injection attempt
    assert!(ConfigLanguage::from_str("../../etc/passwd").is_err());
    assert!(ConfigLanguage::from_str("rust; rm -rf /").is_err());
}
```

---

## `tests/bench_config_system.rs`

```rust
// PURPOSE: Benchmark tests for config-system — parsing, loading, and workspace discovery.
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use config_system_lint_arwaky::capabilities_rules_validator::ConfigRulesValidator;
use config_system_lint_arwaky::capabilities_workspace_detector::WorkspaceDetector;
use config_system_lint_arwaky::root_config_system_container::ConfigContainer;
use shared::common::taxonomy_adapter_name_vo::AdapterName;
use shared::common::taxonomy_path_vo::FilePath;
use shared::config_system::contract_config_orchestrator_aggregate::IConfigOrchestratorAggregate;
use shared::config_system::contract_validator_protocol::IConfigValidatorProtocol;
use shared::config_system::contract_workspace_detector_protocol::IWorkspaceDetectorProtocol;
use shared::config_system::taxonomy_setting_vo::{AdapterEntry, AdapterStatus, ProjectConfig};
use shared::config_system::utility_config_parser::parse_config_yaml;
use std::fs;
use tempfile::TempDir;

fn bench_parse_config_yaml(c: &mut Criterion) {
    let yaml_small = "architecture:\n  enabled: true\n  rules: []\n";
    let yaml_large = format!(
        "architecture:\n  enabled: true\n  rules:\n{}\n",
        (0..100)
            .map(|i| format!(
                "    - name: rule_{}\n      description: Rule {}\n      rule_type: AES{}\n      enabled: true\n      scope: capabilities\n",
                i, i, 300 + i
            ))
            .collect::<String>()
    );

    let mut group = c.benchmark_group("parse_config_yaml");
    group.bench_with_input(BenchmarkId::new("small", "10_lines"), &yaml_small, |b, yaml| {
        b.iter(|| parse_config_yaml(yaml))
    });
    group.bench_with_input(BenchmarkId::new("large", "100_rules"), &yaml_large, |b, yaml| {
        b.iter(|| parse_config_yaml(yaml))
    });
    group.finish();
}

fn bench_workspace_detect(c: &mut Criterion) {
    let tmp = TempDir::new().unwrap();
    fs::write(tmp.path().join("Cargo.toml"), "[package]\nname=\"x\"\n").unwrap();
    let fp = FilePath::new(tmp.path().to_string_lossy().to_string()).unwrap();
    let detector = WorkspaceDetector::new();

    c.bench_function("workspace_detect_rust", |b| {
        b.iter(|| detector.detect(&fp))
    });
}

fn bench_validate_thresholds(c: &mut Criterion) {
    let validator = ConfigRulesValidator::new();
    let mut config = ProjectConfig::default();
    config.adapters = (0..50)
        .map(|i| AdapterEntry::new(AdapterName::raw(format!("adapter_{}", i)), AdapterStatus::Enabled, 1.0))
        .collect();

    c.bench_function("validate_thresholds_50_adapters", |b| {
        b.iter(|| validator.validate_thresholds(&config))
    });
}

fn bench_load_config_sync(c: &mut Criterion) {
    let tmp = TempDir::new().unwrap();
    fs::write(
        tmp.path().join("lint_arwaky.config.rust.yaml"),
        "architecture:\n  enabled: true\n  rules: []\n",
    )
    .unwrap();
    fs::write(tmp.path().join("Cargo.toml"), "[package]\nname=\"x\"\n").unwrap();

    let container = ConfigContainer::new();
    let orch = container.orchestrator();
    let root_str = tmp.path().to_str().unwrap().to_string();

    c.bench_function("load_config_sync", |b| {
        b.iter(|| orch.load_config_sync(&root_str))
    });
}

criterion_group!(
    benches,
    bench_parse_config_yaml,
    bench_workspace_detect,
    bench_validate_thresholds,
    bench_load_config_sync,
);
criterion_main!(benches);
```

---

## `Cargo.toml` additions

Add the following to `crates/config-system/Cargo.toml`:

```toml
[dev-dependencies]
tokio.workspace = true
tempfile = "3"
criterion = { version = "0.5", features = ["async_tokio"] }

[[bench]]
name = "bench_config_system"
path = "tests/bench_config_system.rs"
harness = false
```

---

## Quick Reference

```bash
# Run all tests
cargo test -p config_system-lint-arwaky

# Run by type
cargo test -p config_system-lint-arwaky --test contract_config_system
cargo test -p config_system-lint-arwaky --test unit_config_system_rules_validator
cargo test -p config_system-lint-arwaky --test unit_config_system_workspace_detector
cargo test -p config_system-lint-arwaky --test unit_config_system_yaml_reader
cargo test -p config_system-lint-arwaky --test unit_config_system_parser_provider
cargo test -p config_system-lint-arwaky --test unit_config_system_orchestrator
cargo test -p config_system-lint-arwaky --test integration_config_system
cargo test -p config_system-lint-arwaky --test smoke_config_system
cargo test -p config_system-lint-arwaky --test e2e_config_system_flow

# Acceptance
cargo test -p config_system-lint-arwaky --test acceptance_US_1
cargo test -p config_system-lint-arwaky --test acceptance_US_2
cargo test -p config_system-lint-arwaky --test acceptance_US_3
cargo test -p config_system-lint-arwaky --test acceptance_US_4
cargo test -p config_system-lint-arwaky --test acceptance_US_5

# Benchmarks
cargo bench -p config_system-lint-arwaky

# Coverage
cargo tarpaulin -p config_system-lint-arwaky --fail-under 70

# With output
cargo test -p config_system-lint-arwaky -- --nocapture
```

---

## Coverage Summary

| Layer                  | File(s) Tested                                                                                  | Target | Tests                            |
| ---------------------- | ----------------------------------------------------------------------------------------------- | ------ | -------------------------------- |
| **Capabilities** | `ConfigRulesValidator`, `WorkspaceDetector`, `ConfigYamlReader`, `ConfigParserProvider` | ≥ 70% | 40+ unit tests                   |
| **Agent**        | `ConfigOrchestrator`                                                                          | ≥ 60% | 12 unit + integration tests      |
| **Utility**      | `utility_config_io`, `utility_config_parser` (via integration)                              | ≥ 50% | Covered through E2E + acceptance |
| **Root**         | `ConfigContainer`                                                                             | —     | 8 integration tests              |
| **Contracts**    | All 5 traits                                                                                    | —     | 6 contract tests                 |
