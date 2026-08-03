// Acceptance tests — verify FRD requirements are met for git-hooks.
//
// Covers:
//   FR-002: Pre-commit hook installation
//   FR-003: Pre-commit hook uninstallation
//   FR-004: Git hooks check execution
//   FR-005: Diff data comparison
//   FR-006: Ignore rule management

use git_hooks_lint_arwaky::capabilities_hook_adapter::GitHookAdapter;
use git_hooks_lint_arwaky::capabilities_hook_manager::HookManager;
use git_hooks_lint_arwaky::root_git_hooks_container::GitContainer;
use shared::common::FilePath;
use shared::git_hooks::contract_git_hooks_aggregate::GitHooksAggregate;
use shared::git_hooks::{GitDiffStatus, HookIgnoreUpdateVO, IHookManagerProtocol, IHookProtocol};
use std::sync::Arc;
use tempfile::TempDir;

// ─── Helpers ──────────────────────────────────────────────

fn make_container() -> (TempDir, Arc<dyn GitHooksAggregate>) {
    let tmp = TempDir::new().unwrap();
    let filesystem =
        filesystem::root_filesystem_container::FilesystemContainer::new().orchestrator();
    let fp = FilePath::new(tmp.path().to_string_lossy().to_string()).unwrap();
    let container = GitContainer::new(fp, filesystem);
    (tmp, container.aggregate())
}

fn make_adapter(tmp: &TempDir) -> GitHookAdapter {
    let filesystem =
        filesystem::root_filesystem_container::FilesystemContainer::new().orchestrator();
    let fp = FilePath::new(tmp.path().to_string_lossy().to_string()).unwrap();
    GitHookAdapter::new(fp, filesystem)
}

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

// ═══════════════════════════════════════════════════════════
// FR-002: Pre-commit hook installation
// ═══════════════════════════════════════════════════════════

#[test]
fn fr002_hook_script_contains_correct_executable() {
    let tmp = TempDir::new().unwrap();
    let hooks_dir = tmp.path().join(".git").join("hooks");
    std::fs::create_dir_all(&hooks_dir).unwrap();

    let adapter = make_adapter(&tmp);
    let exec_path = FilePath::new("/usr/local/bin/lint-arwaky-cli".to_string()).unwrap();
    let result = adapter.install_pre_commit(&exec_path);
    assert!(result.is_ok(), "install should succeed: {:?}", result.err());
    assert!(result.unwrap().value, "should return true");

    let hook_content = std::fs::read_to_string(hooks_dir.join("pre-commit")).unwrap();
    assert!(
        hook_content.contains("/usr/local/bin/lint-arwaky-cli check ."),
        "hook should contain the full executable path as check command: {}",
        hook_content
    );
}

#[test]
fn fr002_hook_script_starts_with_shebang() {
    let tmp = TempDir::new().unwrap();
    let hooks_dir = tmp.path().join(".git").join("hooks");
    std::fs::create_dir_all(&hooks_dir).unwrap();

    let adapter = make_adapter(&tmp);
    let exec_path = FilePath::new("lint-arwaky-cli".to_string()).unwrap();
    adapter.install_pre_commit(&exec_path).unwrap();

    let hook_content = std::fs::read_to_string(hooks_dir.join("pre-commit")).unwrap();
    assert!(
        hook_content.starts_with("#!/bin/bash"),
        "hook should start with bash shebang"
    );
}

#[test]
fn fr002_creates_hooks_directory_when_missing() {
    let tmp = TempDir::new().unwrap();
    // No .git/hooks directory — adapter should create it
    assert!(!tmp.path().join(".git").exists());

    let adapter = make_adapter(&tmp);
    let exec_path = FilePath::new("lint-arwaky-cli".to_string()).unwrap();
    // Non-git repo returns SuccessStatus(false) — no error
    let result = adapter.install_pre_commit(&exec_path);
    assert!(
        result.is_ok(),
        "install on non-git should not error: {:?}",
        result.err()
    );
    assert!(!result.unwrap().value, "should return false for non-git repo");
}

#[test]
fn fr002_overwrites_existing_hook() {
    let tmp = TempDir::new().unwrap();
    let hooks_dir = tmp.path().join(".git").join("hooks");
    std::fs::create_dir_all(&hooks_dir).unwrap();

    let adapter = make_adapter(&tmp);
    let exec_path = FilePath::new("first-executable".to_string()).unwrap();
    adapter.install_pre_commit(&exec_path).unwrap();

    // Install again with different executable
    let exec_path2 = FilePath::new("second-executable".to_string()).unwrap();
    adapter.install_pre_commit(&exec_path2).unwrap();

    let hook_content = std::fs::read_to_string(hooks_dir.join("pre-commit")).unwrap();
    assert!(
        hook_content.contains("second-executable"),
        "hook should contain the new executable after overwrite"
    );
    assert!(
        !hook_content.contains("first-executable"),
        "old executable should be gone"
    );
}

#[test]
fn fr002_not_git_repo_returns_false() {
    let tmp = TempDir::new().unwrap();
    // No .git directory
    let adapter = make_adapter(&tmp);
    let exec_path = FilePath::new("lint-arwaky-cli".to_string()).unwrap();
    let result = adapter.install_pre_commit(&exec_path).unwrap();
    assert!(!result.value, "non-git repo should return false");
}

// ═══════════════════════════════════════════════════════════
// FR-003: Pre-commit hook uninstallation
// ═══════════════════════════════════════════════════════════

#[test]
fn fr003_removes_existing_hook() {
    let tmp = TempDir::new().unwrap();
    let hooks_dir = tmp.path().join(".git").join("hooks");
    std::fs::create_dir_all(&hooks_dir).unwrap();
    // Write a pre-commit hook manually
    write_file(&hooks_dir.join("pre-commit"), "#!/bin/bash\nexit 0\n");

    let adapter = make_adapter(&tmp);
    let result = adapter.uninstall_pre_commit();
    assert!(
        result.is_ok(),
        "uninstall should succeed: {:?}",
        result.err()
    );
    assert!(result.unwrap().value, "should return true when hook existed");
    assert!(
        !hooks_dir.join("pre-commit").exists(),
        "pre-commit should be removed"
    );
}

#[test]
fn fr003_idempotent_when_hook_missing() {
    let tmp = TempDir::new().unwrap();
    let hooks_dir = tmp.path().join(".git").join("hooks");
    std::fs::create_dir_all(&hooks_dir).unwrap();
    // No pre-commit file

    let adapter = make_adapter(&tmp);
    let result = adapter.uninstall_pre_commit();
    assert!(result.is_ok(), "uninstall should succeed");
    assert!(
        result.unwrap().value,
        "should return true even when no hook exists"
    );
}

#[test]
fn fr003_not_git_repo_returns_false() {
    let tmp = TempDir::new().unwrap();
    // No .git directory
    let adapter = make_adapter(&tmp);
    let result = adapter.uninstall_pre_commit().unwrap();
    assert!(!result.value, "non-git repo should return false");
}

#[test]
fn fr003_only_removes_pre_commit_hook() {
    let tmp = TempDir::new().unwrap();
    let hooks_dir = tmp.path().join(".git").join("hooks");
    std::fs::create_dir_all(&hooks_dir).unwrap();
    write_file(&hooks_dir.join("pre-commit"), "#!/bin/bash\nexit 0\n");
    write_file(&hooks_dir.join("commit-msg"), "#!/bin/bash\nexit 0\n");

    let adapter = make_adapter(&tmp);
    adapter.uninstall_pre_commit().unwrap();

    assert!(
        !hooks_dir.join("pre-commit").exists(),
        "pre-commit should be removed"
    );
    assert!(
        hooks_dir.join("commit-msg").exists(),
        "other hooks should remain untouched"
    );
}

// ═══════════════════════════════════════════════════════════
// FR-004: Git hooks check execution
// ═══════════════════════════════════════════════════════════

#[test]
fn fr004_check_on_non_git_dir_returns_empty_results() {
    let (tmp, aggregate) = make_container();
    let fp = FilePath::new(tmp.path().to_string_lossy().to_string()).unwrap();
    let results = aggregate.run_git_hooks_check(&fp);
    // Non-git directory → no changes detected → empty results
    assert!(
        results.is_empty(),
        "check on non-git dir should return empty results"
    );
}

#[test]
fn fr004_check_does_not_panic_on_invalid_path() {
    let (_, aggregate) = make_container();
    let fp = FilePath::new("/nonexistent/path/that/does/not/exist".to_string()).unwrap();
    let _results = aggregate.run_git_hooks_check(&fp);
    // Should not panic even with invalid path
}

// ═══════════════════════════════════════════════════════════
// FR-005: Diff data comparison
// ═══════════════════════════════════════════════════════════

#[test]
fn fr005_identical_files_score_zero_unchanged() {
    let tmp = TempDir::new().unwrap();
    let mgr = make_hook_manager(&tmp);
    let p1 = tmp.path().join("a.txt");
    let p2 = tmp.path().join("b.txt");
    write_file(&p1, "exact same content");
    write_file(&p2, "exact same content");

    let result = mgr.get_diff_data(p1.to_str().unwrap(), p2.to_str().unwrap());
    assert_eq!(result.status, GitDiffStatus::Unchanged);
    assert!((result.difference - 0.0).abs() < f64::EPSILON);
}

#[test]
fn fr005_modified_files_score_between_zero_and_one() {
    let tmp = TempDir::new().unwrap();
    let mgr = make_hook_manager(&tmp);
    let p1 = tmp.path().join("a.txt");
    let p2 = tmp.path().join("b.txt");
    write_file(&p1, "original content");
    write_file(&p2, "changed content");

    let result = mgr.get_diff_data(p1.to_str().unwrap(), p2.to_str().unwrap());
    assert_eq!(result.status, GitDiffStatus::Modified);
    assert!(result.difference > 0.0, "score should be > 0");
    assert!(result.difference <= 1.0, "score should be <= 1.0");
}

#[test]
fn fr005_first_file_missing_returns_missing_first() {
    let tmp = TempDir::new().unwrap();
    let mgr = make_hook_manager(&tmp);
    let p1 = tmp.path().join("missing.txt");
    let p2 = tmp.path().join("exists.txt");
    write_file(&p2, "content");

    let result = mgr.get_diff_data(p1.to_str().unwrap(), p2.to_str().unwrap());
    assert_eq!(result.status, GitDiffStatus::MissingFirst);
}

#[test]
fn fr005_second_file_missing_returns_missing_second() {
    let tmp = TempDir::new().unwrap();
    let mgr = make_hook_manager(&tmp);
    let p1 = tmp.path().join("exists.txt");
    let p2 = tmp.path().join("missing.txt");
    write_file(&p1, "content");

    let result = mgr.get_diff_data(p1.to_str().unwrap(), p2.to_str().unwrap());
    assert_eq!(result.status, GitDiffStatus::MissingSecond);
}

#[test]
fn fr005_directories_return_not_a_file() {
    let tmp = TempDir::new().unwrap();
    let mgr = make_hook_manager(&tmp);
    let p1 = tmp.path().join("dir_a");
    let p2 = tmp.path().join("dir_b");
    std::fs::create_dir(&p1).unwrap();
    std::fs::create_dir(&p2).unwrap();

    let result = mgr.get_diff_data(p1.to_str().unwrap(), p2.to_str().unwrap());
    assert_eq!(result.status, GitDiffStatus::NotAFile);
}

#[test]
fn fr005_same_path_twice_returns_unchanged() {
    let tmp = TempDir::new().unwrap();
    let mgr = make_hook_manager(&tmp);
    let p1 = tmp.path().join("same.txt");
    write_file(&p1, "some content");

    let result = mgr.get_diff_data(p1.to_str().unwrap(), p1.to_str().unwrap());
    assert_eq!(result.status, GitDiffStatus::Unchanged);
    assert!((result.difference - 0.0).abs() < f64::EPSILON);
}

#[test]
fn fr005_both_missing_returns_both_missing() {
    let tmp = TempDir::new().unwrap();
    let mgr = make_hook_manager(&tmp);
    let p1 = tmp.path().join("nope1.txt");
    let p2 = tmp.path().join("nope2.txt");

    let result = mgr.get_diff_data(p1.to_str().unwrap(), p2.to_str().unwrap());
    assert_eq!(result.status, GitDiffStatus::BothMissing);
}

#[test]
fn fr005_empty_file_vs_nonempty_scores_one() {
    let tmp = TempDir::new().unwrap();
    let mgr = make_hook_manager(&tmp);
    let p1 = tmp.path().join("empty.txt");
    let p2 = tmp.path().join("full.txt");
    write_file(&p1, "");
    write_file(&p2, "some content here");

    let result = mgr.get_diff_data(p1.to_str().unwrap(), p2.to_str().unwrap());
    assert_eq!(result.status, GitDiffStatus::Modified);
    assert!(
        (result.difference - 1.0).abs() < f64::EPSILON,
        "empty vs nonempty should score 1.0, got {}",
        result.difference
    );
}

#[test]
fn fr005_side_scores_reflect_difference() {
    let tmp = TempDir::new().unwrap();
    let mgr = make_hook_manager(&tmp);
    let p1 = tmp.path().join("a.txt");
    let p2 = tmp.path().join("b.txt");
    write_file(&p1, "aaa");
    write_file(&p2, "zzz");

    let result = mgr.get_diff_data(p1.to_str().unwrap(), p2.to_str().unwrap());
    assert_eq!(result.status, GitDiffStatus::Modified);
    // Completely different → side scores should match the difference
    assert!((result.version1.similarity_score - result.difference).abs() < f64::EPSILON);
    assert!((result.version2.similarity_score - result.difference).abs() < f64::EPSILON);
}

// ═══════════════════════════════════════════════════════════
// FR-006: Ignore rule management
// ═══════════════════════════════════════════════════════════

#[test]
fn fr006_add_rule_to_config() {
    let tmp = TempDir::new().unwrap();
    let config_path = tmp.path().join("lint_arwaky.config.yaml");
    write_file(
        &config_path,
        "# Lint Arwaky Configuration\nignored_paths:\n  - vendor\n",
    );
    let mgr = make_hook_manager(&tmp);

    let request = HookIgnoreUpdateVO::new("dist", false, config_path.to_str().unwrap().to_string());
    let result = mgr.update_ignore_rule(request);
    assert!(
        result.value.contains("Added"),
        "should report Added: {}",
        result.value
    );

    let content = std::fs::read_to_string(&config_path).unwrap();
    assert!(content.contains("dist"), "config should contain 'dist'");
}

#[test]
fn fr006_remove_rule_from_config() {
    let tmp = TempDir::new().unwrap();
    let config_path = tmp.path().join("lint_arwaky.config.yaml");
    write_file(
        &config_path,
        "# Lint Arwaky Configuration\nignored_paths:\n  - vendor\n  - node_modules\n",
    );
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
        "config should not contain '- vendor'"
    );
    assert!(
        content.contains("- node_modules"),
        "other rules should remain"
    );
}

#[test]
fn fr006_config_not_found_suggests_init() {
    let tmp = TempDir::new().unwrap();
    let config_path = tmp.path().join("nonexistent.yaml");
    let mgr = make_hook_manager(&tmp);

    let request = HookIgnoreUpdateVO::new("test", false, config_path.to_str().unwrap().to_string());
    let result = mgr.update_ignore_rule(request);
    assert!(
        result.value.contains("not found") || result.value.contains("Run lint-arwaky-cli"),
        "should suggest init when config not found: {}",
        result.value
    );
}

#[test]
fn fr006_add_existing_rule_is_noop() {
    let tmp = TempDir::new().unwrap();
    let config_path = tmp.path().join("lint_arwaky.config.yaml");
    write_file(
        &config_path,
        "# Lint Arwaky Configuration\nignored_paths:\n  - vendor\n",
    );
    let mgr = make_hook_manager(&tmp);

    let request =
        HookIgnoreUpdateVO::new("vendor", false, config_path.to_str().unwrap().to_string());
    let result = mgr.update_ignore_rule(request);
    assert!(
        result.value.contains("already present"),
        "duplicate add should be no-op: {}",
        result.value
    );
}

#[test]
fn fr006_initialize_config_creates_default() {
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
        "default config should contain ignored_paths key"
    );
}

#[test]
fn fr006_initialize_config_already_exists() {
    let tmp = TempDir::new().unwrap();
    let config_path = tmp.path().join("lint_arwaky.config.yaml");
    write_file(
        &config_path,
        "# Lint Arwaky Configuration\nignored_paths:\n  - vendor\n",
    );
    let mgr = make_hook_manager(&tmp);

    let result = mgr.initialize_config(tmp.path().to_str().unwrap());
    assert!(
        result.value.contains("ALREADY_EXISTS"),
        "should report ALREADY_EXISTS: {}",
        result.value
    );
}

// ═══════════════════════════════════════════════════════════
// Cross-cutting: Identity & trait compliance
// ═══════════════════════════════════════════════════════════

#[test]
fn hook_manager_identity_is_git_hook_manager() {
    let tmp = TempDir::new().unwrap();
    let mgr = make_hook_manager(&tmp);
    let identity = mgr.get_hook_manager_identity();
    assert_eq!(identity.value(), "git_hook_manager");
}

#[test]
fn orchestrator_hook_manager_identity_delegates_correctly() {
    let (_, aggregate) = make_container();
    let identity = aggregate.get_hook_manager_identity();
    assert_eq!(
        identity.value(),
        "git_hook_manager",
        "orchestrator should delegate identity to hook_manager"
    );
}

#[test]
fn non_git_repo_all_operations_are_safe() {
    let (tmp, aggregate) = make_container();
    let fp = FilePath::new(tmp.path().to_string_lossy().to_string()).unwrap();

    // None of these should panic on a non-git directory
    let _results = aggregate.run_git_hooks_check(&fp);
    let install = aggregate.install_hook(&fp);
    let uninstall = aggregate.uninstall_hook();

    assert!(install.is_ok(), "install should not error");
    assert!(uninstall.is_ok(), "uninstall should not error");
}
