// Unit tests — HookManager: get_diff_data, initialize_config, update_ignore_rule.

use git_hooks_lint_arwaky::capabilities_hook_adapter::GitHookAdapter;
use git_hooks_lint_arwaky::capabilities_hook_manager::HookManager;
use shared::common::FilePath;
use shared::git_hooks::{GitDiffStatus, HookIgnoreUpdateVO, IHookManagerProtocol, IHookProtocol};
use std::sync::Arc;
use tempfile::TempDir;

fn make_hook_manager(tmp: &TempDir) -> HookManager {
    let filesystem =
        filesystem::root_filesystem_container::FilesystemContainer::new().orchestrator();
    let fp = FilePath::new(tmp.path().to_string_lossy().to_string()).unwrap();
    let adapter: Arc<dyn IHookManagerProtocol> =
        Arc::new(GitHookAdapter::new(fp, filesystem.clone()));
    HookManager::new(adapter, filesystem)
}

fn write_file(path: &std::path::Path, content: &str) {
    std::fs::write(path, content).unwrap();
}

// ─── FR-005: Diff Data Comparison ─────────────────────────

#[test]
fn fr005_1_both_files_identical() {
    let tmp = TempDir::new().unwrap();
    let p1 = tmp.path().join("file1.txt");
    let p2 = tmp.path().join("file2.txt");
    write_file(&p1, "hello world");
    write_file(&p2, "hello world");
    let mgr = make_hook_manager(&tmp);
    let result = mgr.get_diff_data(p1.to_str().unwrap(), p2.to_str().unwrap());
    assert_eq!(result.status, GitDiffStatus::Unchanged);
    assert!((result.difference - 0.0).abs() < f64::EPSILON);
}

#[test]
fn fr005_2_files_partially_different() {
    let tmp = TempDir::new().unwrap();
    let p1 = tmp.path().join("file1.txt");
    let p2 = tmp.path().join("file2.txt");
    write_file(&p1, "hello world");
    write_file(&p2, "hello earth");
    let mgr = make_hook_manager(&tmp);
    let result = mgr.get_diff_data(p1.to_str().unwrap(), p2.to_str().unwrap());
    assert_eq!(result.status, GitDiffStatus::Modified);
    assert!(result.difference > 0.0);
    assert!(result.difference <= 1.0);
}

#[test]
fn fr005_3_first_file_missing() {
    let tmp = TempDir::new().unwrap();
    let p1 = tmp.path().join("missing1.txt");
    let p2 = tmp.path().join("file2.txt");
    write_file(&p2, "content");
    let mgr = make_hook_manager(&tmp);
    let result = mgr.get_diff_data(p1.to_str().unwrap(), p2.to_str().unwrap());
    assert_eq!(result.status, GitDiffStatus::MissingFirst);
}

#[test]
fn fr005_4_second_file_missing() {
    let tmp = TempDir::new().unwrap();
    let p1 = tmp.path().join("file1.txt");
    let p2 = tmp.path().join("missing2.txt");
    write_file(&p1, "content");
    let mgr = make_hook_manager(&tmp);
    let result = mgr.get_diff_data(p1.to_str().unwrap(), p2.to_str().unwrap());
    assert_eq!(result.status, GitDiffStatus::MissingSecond);
}

#[test]
fn fr005_5_both_paths_are_directories() {
    let tmp = TempDir::new().unwrap();
    let p1 = tmp.path().join("dir1");
    let p2 = tmp.path().join("dir2");
    std::fs::create_dir(&p1).unwrap();
    std::fs::create_dir(&p2).unwrap();
    let mgr = make_hook_manager(&tmp);
    let result = mgr.get_diff_data(p1.to_str().unwrap(), p2.to_str().unwrap());
    assert_eq!(result.status, GitDiffStatus::NotAFile);
}

#[test]
fn fr005_6_same_file_path_twice() {
    let tmp = TempDir::new().unwrap();
    let p1 = tmp.path().join("same.txt");
    write_file(&p1, "content");
    let mgr = make_hook_manager(&tmp);
    let result = mgr.get_diff_data(p1.to_str().unwrap(), p1.to_str().unwrap());
    assert_eq!(result.status, GitDiffStatus::Unchanged);
    assert!((result.difference - 0.0).abs() < f64::EPSILON);
}

#[test]
fn fr005_both_paths_missing() {
    let tmp = TempDir::new().unwrap();
    let p1 = tmp.path().join("nonexistent1.txt");
    let p2 = tmp.path().join("nonexistent2.txt");
    let mgr = make_hook_manager(&tmp);
    let result = mgr.get_diff_data(p1.to_str().unwrap(), p2.to_str().unwrap());
    assert_eq!(result.status, GitDiffStatus::BothMissing);
}

#[test]
fn fr005_side_scores_match_difference() {
    let tmp = TempDir::new().unwrap();
    let p1 = tmp.path().join("a.txt");
    let p2 = tmp.path().join("b.txt");
    write_file(&p1, "abc");
    write_file(&p2, "xyz");
    let mgr = make_hook_manager(&tmp);
    let result = mgr.get_diff_data(p1.to_str().unwrap(), p2.to_str().unwrap());
    // Completely different content → side scores should reflect difference
    assert_eq!(result.status, GitDiffStatus::Modified);
    assert!((result.version1.similarity_score - result.difference).abs() < f64::EPSILON);
    assert!((result.version2.similarity_score - result.difference).abs() < f64::EPSILON);
}

#[test]
fn fr005_identical_files_have_zero_side_scores() {
    let tmp = TempDir::new().unwrap();
    let p1 = tmp.path().join("a.txt");
    let p2 = tmp.path().join("b.txt");
    write_file(&p1, "same content");
    write_file(&p2, "same content");
    let mgr = make_hook_manager(&tmp);
    let result = mgr.get_diff_data(p1.to_str().unwrap(), p2.to_str().unwrap());
    assert_eq!(result.status, GitDiffStatus::Unchanged);
    assert!((result.version1.similarity_score - 0.0).abs() < f64::EPSILON);
    assert!((result.version2.similarity_score - 0.0).abs() < f64::EPSILON);
}

// ─── FR-006: Ignore Rule Management ───────────────────────

fn write_yaml_config(path: &std::path::Path) {
    write_file(
        path,
        "# Lint Arwaky Configuration\nignored_paths:\n  - vendor\n  - node_modules\n",
    );
}

#[test]
fn fr006_1_add_ignore_rule() {
    let tmp = TempDir::new().unwrap();
    let config_path = tmp.path().join("lint_arwaky.config.yaml");
    write_yaml_config(&config_path);
    let mgr = make_hook_manager(&tmp);
    let request =
        HookIgnoreUpdateVO::new("target", false, config_path.to_str().unwrap().to_string());
    let result = mgr.update_ignore_rule(request);
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
fn fr006_2_remove_ignore_rule() {
    let tmp = TempDir::new().unwrap();
    let config_path = tmp.path().join("lint_arwaky.config.yaml");
    write_yaml_config(&config_path);
    let mgr = make_hook_manager(&tmp);
    let request =
        HookIgnoreUpdateVO::new("vendor", true, config_path.to_str().unwrap().to_string());
    let result = mgr.update_ignore_rule(request);
    assert!(
        result.value.contains("Removed"),
        "should report Removed: {}",
        result.value
    );
    let content = std::fs::read_to_string(&config_path).unwrap();
    assert!(
        !content.contains("- vendor"),
        "config should not contain 'vendor'"
    );
}

#[test]
fn fr006_3_config_file_not_found() {
    let tmp = TempDir::new().unwrap();
    let config_path = tmp.path().join("nonexistent.yaml");
    let mgr = make_hook_manager(&tmp);
    let request = HookIgnoreUpdateVO::new("rule", false, config_path.to_str().unwrap().to_string());
    let result = mgr.update_ignore_rule(request);
    assert!(
        result.value.contains("not found") || result.value.contains("Run lint-arwaky-cli"),
        "should report not found: {}",
        result.value
    );
}

#[test]
fn fr006_4_rule_already_exists_add_noop() {
    let tmp = TempDir::new().unwrap();
    let config_path = tmp.path().join("lint_arwaky.config.yaml");
    write_yaml_config(&config_path);
    let mgr = make_hook_manager(&tmp);
    let request =
        HookIgnoreUpdateVO::new("vendor", false, config_path.to_str().unwrap().to_string());
    let result = mgr.update_ignore_rule(request);
    assert!(
        result.value.contains("already present"),
        "should report already present: {}",
        result.value
    );
}

// ─── FR-006: initialize_config ────────────────────────────

#[test]
fn initialize_config_creates_config_file() {
    let tmp = TempDir::new().unwrap();
    let mgr = make_hook_manager(&tmp);
    let result = mgr.initialize_config(tmp.path().to_str().unwrap());
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
    let mgr = make_hook_manager(&tmp);
    let result = mgr.initialize_config(tmp.path().to_str().unwrap());
    assert!(
        result.value.contains("ALREADY_EXISTS"),
        "should report ALREADY_EXISTS: {}",
        result.value
    );
}

// ─── Trait method identity ────────────────────────────────

#[test]
fn get_hook_manager_identity_returns_correct_id() {
    let tmp = TempDir::new().unwrap();
    let mgr = make_hook_manager(&tmp);
    let identity = mgr.get_hook_manager_identity();
    assert_eq!(identity.value(), "git_hook_manager");
}
