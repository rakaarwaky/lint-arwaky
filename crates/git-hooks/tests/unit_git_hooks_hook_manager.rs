// Unit tests — ConfigInit: initialize_config, update_ignore_rule.

use git_hooks_lint_arwaky::capabilities_config_init::ConfigInit;
use shared::git_hooks::{HookIgnoreUpdateVO, IConfigInitProtocol};
use tempfile::TempDir;

fn make_config_init() -> ConfigInit {
    let fs_container = filesystem::root_filesystem_container::FilesystemContainer::new();
    let io = fs_container.io();
    ConfigInit::new(io)
}

fn write_file(path: &std::path::Path, content: &str) {
    std::fs::write(path, content).unwrap();
}

// ─── FR-GitHooks-004: Config Initialization ────────────────

fn write_yaml_config(path: &std::path::Path) {
    write_file(
        path,
        "# Lint Arwaky Configuration\nignored_paths:\n  - vendor\n  - node_modules\n",
    );
}

#[test]
fn initialize_config_creates_config_file() {
    let tmp = TempDir::new().unwrap();
    let config_init = make_config_init();
    let result = config_init.initialize_config(tmp.path().to_str().unwrap());
    assert!(
        result.value.contains("Initialized"),
        "should report Initialized: {}",
        result.value
    );
    let config_path = tmp.path().join("lint_arwaky.config.yaml");
    assert!(config_path.exists(), "config file should be created");
    let content = std::fs::read_to_string(&config_path).unwrap();
    assert!(
        content.contains("ignored_paths"),
        "config should contain ignored_paths key"
    );
}

#[test]
fn initialize_config_already_exists() {
    let tmp = TempDir::new().unwrap();
    let config_path = tmp.path().join("lint_arwaky.config.yaml");
    write_yaml_config(&config_path);
    let config_init = make_config_init();
    let result = config_init.initialize_config(tmp.path().to_str().unwrap());
    assert!(
        result.value.contains("ALREADY_EXISTS"),
        "should report ALREADY_EXISTS: {}",
        result.value
    );
}

// ─── FR-GitHooks-004: Ignore Rule Management ───────────────

#[test]
fn add_ignore_rule() {
    let tmp = TempDir::new().unwrap();
    let config_path = tmp.path().join("lint_arwaky.config.yaml");
    write_yaml_config(&config_path);
    let config_init = make_config_init();
    let request =
        HookIgnoreUpdateVO::new("target", false, config_path.to_str().unwrap().to_string());
    let result = config_init.update_ignore_rule(request);
    assert!(
        result.value.contains("Added"),
        "should report Added: {}",
        result.value
    );
    // Verify the rule was added
    let content = std::fs::read_to_string(&config_path).unwrap();
    assert!(content.contains("target"), "config should contain 'target'");
}

#[test]
fn remove_ignore_rule() {
    let tmp = TempDir::new().unwrap();
    let config_path = tmp.path().join("lint_arwaky.config.yaml");
    write_yaml_config(&config_path);
    let config_init = make_config_init();
    let request =
        HookIgnoreUpdateVO::new("vendor", true, config_path.to_str().unwrap().to_string());
    let result = config_init.update_ignore_rule(request);
    assert!(
        result.value.contains("Removed"),
        "should report Removed: {}",
        result.value
    );
    let content = std::fs::read_to_string(&config_path).unwrap();
    assert!(
        !content.contains("- vendor"),
        "config should not contain '- vendor'"
    );
}

#[test]
fn config_file_not_found() {
    let tmp = TempDir::new().unwrap();
    let config_path = tmp.path().join("nonexistent.yaml");
    let config_init = make_config_init();
    let request = HookIgnoreUpdateVO::new("rule", false, config_path.to_str().unwrap().to_string());
    let result = config_init.update_ignore_rule(request);
    assert!(
        result.value.contains("not found") || result.value.contains("Run lint-arwaky-cli"),
        "should report not found: {}",
        result.value
    );
}

#[test]
fn rule_already_exists_add_noop() {
    let tmp = TempDir::new().unwrap();
    let config_path = tmp.path().join("lint_arwaky.config.yaml");
    write_yaml_config(&config_path);
    let config_init = make_config_init();
    let request =
        HookIgnoreUpdateVO::new("vendor", false, config_path.to_str().unwrap().to_string());
    let result = config_init.update_ignore_rule(request);
    assert!(
        result.value.contains("already present"),
        "should report already present: {}",
        result.value
    );
}
