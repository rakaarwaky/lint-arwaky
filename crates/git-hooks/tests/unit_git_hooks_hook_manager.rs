// PURPOSE: Unit tests for HookManager — IHookProtocol implementation.
// Covers: install/uninstall delegation, identity, config init, ignore rules, diff data.
// Layer: Capabilities (HookManager)
// Speed: ms

use git_hooks_lint_arwaky::capabilities_hook_adapter::GitHookAdapter;
use git_hooks_lint_arwaky::capabilities_hook_manager::HookManager;
use shared::common::taxonomy_path_vo::FilePath;
use shared::git_hooks::contract_hook_protocol::IHookProtocol;
use shared::git_hooks::contract_manager_protocol::IHookManagerProtocol;
use shared::git_hooks::taxonomy_git_diff_data_vo::{GitDiffStatus, HookIgnoreUpdateVO};
use std::sync::Arc;

fn sut() -> HookManager {
    let adapter: Arc<dyn IHookManagerProtocol> = Arc::new(GitHookAdapter::new(
        FilePath::new("/tmp/nonexistent_repo").unwrap_or_default(),
    ));
    HookManager::new(adapter)
}

// ─── Construction ─────────────────────────────────────────

#[test]
fn new_creates_instance() {
    let _manager = sut();
}

// ─── get_hook_manager_identity ────────────────────────────

#[test]
fn get_hook_manager_identity_returns_expected_name() {
    let manager = sut();
    let identity = manager.get_hook_manager_identity();
    assert_eq!(identity.value(), "git_hook_manager");
}

// ─── install_pre_commit (delegates to adapter) ────────────

#[tokio::test]
async fn install_pre_commit_delegates_to_adapter() {
    let manager = sut();
    let exe_path = FilePath::new("/usr/bin/lint-arwaky").unwrap_or_default();
    let result = manager.install_pre_commit(&exe_path).await;
    // Non-repo path → adapter returns Ok(false)
    assert!(result.is_ok());
    assert!(!result.unwrap().value());
}

// ─── uninstall_pre_commit (delegates to adapter) ──────────

#[tokio::test]
async fn uninstall_pre_commit_delegates_to_adapter() {
    let manager = sut();
    let result = manager.uninstall_pre_commit().await;
    assert!(result.is_ok());
    assert!(!result.unwrap().value());
}

// ─── initialize_config ────────────────────────────────────

#[tokio::test]
async fn initialize_config_new_path_returns_initialized() {
    let manager = sut();
    let result = manager.initialize_config("/tmp/nonexistent_project_xyz").await;
    assert!(result.value().starts_with("Initialized"));
}

#[tokio::test]
async fn initialize_config_existing_file_returns_already_exists() {
    // Create a temp config file
    let tmp_dir = std::env::temp_dir().join(format!("git_hooks_config_{}", std::process::id()));
    let _ = std::fs::create_dir_all(&tmp_dir);
    let config_path = tmp_dir.join("lint_arwaky.config.yaml");
    std::fs::write(&config_path, "# config").unwrap();

    let manager = sut();
    let result = manager.initialize_config(tmp_dir.to_str().unwrap()).await;
    assert!(result.value().starts_with("ALREADY_EXISTS:"));

    let _ = std::fs::remove_dir_all(&tmp_dir);
}

// ─── update_ignore_rule ───────────────────────────────────

#[test]
fn update_ignore_rule_missing_config_returns_not_found() {
    let manager = sut();
    let request = HookIgnoreUpdateVO::new("*.log", false, "/tmp/no_config_here.yaml");
    let result = manager.update_ignore_rule(request);
    assert!(result.value().contains("not found"));
}

#[test]
fn update_ignore_rule_add_rule() {
    let tmp_dir = std::env::temp_dir().join(format!("git_hooks_ignore_{}", std::process::id()));
    let _ = std::fs::create_dir_all(&tmp_dir);
    let config_path = tmp_dir.join("lint_arwaky.config.yaml");
    std::fs::write(&config_path, "# config").unwrap();

    let manager = sut();
    let request = HookIgnoreUpdateVO::new(
        "*.generated.rs",
        false,
        config_path.to_str().unwrap().to_string(),
    );
    let result = manager.update_ignore_rule(request);
    assert!(result.value().contains("Added"));
    assert!(result.value().contains("*.generated.rs"));

    let _ = std::fs::remove_dir_all(&tmp_dir);
}

#[test]
fn update_ignore_rule_remove_rule() {
    let tmp_dir = std::env::temp_dir().join(format!("git_hooks_ignore_rm_{}", std::process::id()));
    let _ = std::fs::create_dir_all(&tmp_dir);
    let config_path = tmp_dir.join("lint_arwaky.config.yaml");
    std::fs::write(&config_path, "# config").unwrap();

    let manager = sut();
    let request = HookIgnoreUpdateVO::new(
        "*.generated.rs",
        true,
        config_path.to_str().unwrap().to_string(),
    );
    let result = manager.update_ignore_rule(request);
    assert!(result.value().contains("Removed"));

    let _ = std::fs::remove_dir_all(&tmp_dir);
}

// ─── get_diff_data ────────────────────────────────────────

#[tokio::test]
async fn get_diff_data_both_missing_returns_missing_first() {
    let manager = sut();
    let result = manager
        .get_diff_data("/tmp/no_file_a_xyz", "/tmp/no_file_b_xyz")
        .await;
    assert_eq!(result.status, GitDiffStatus::MissingFirst);
}

#[tokio::test]
async fn get_diff_data_first_exists_second_missing() {
    let tmp_file = std::env::temp_dir().join(format!("diff_test_a_{}", std::process::id()));
    std::fs::write(&tmp_file, "content").unwrap();

    let manager = sut();
    let result = manager
        .get_diff_data(tmp_file.to_str().unwrap(), "/tmp/no_file_b_xyz")
        .await;
    assert_eq!(result.status, GitDiffStatus::MissingSecond);

    let _ = std::fs::remove_file(&tmp_file);
}

#[tokio::test]
async fn get_diff_data_both_exist_same_content_returns_unchanged() {
    let tmp_a = std::env::temp_dir().join(format!("diff_test_same_a_{}", std::process::id()));
    let tmp_b = std::env::temp_dir().join(format!("diff_test_same_b_{}", std::process::id()));
    std::fs::write(&tmp_a, "same content").unwrap();
    std::fs::write(&tmp_b, "same content").unwrap();

    let manager = sut();
    let result = manager
        .get_diff_data(tmp_a.to_str().unwrap(), tmp_b.to_str().unwrap())
        .await;
    // Current impl returns Unchanged when both exist and are files
    assert_eq!(result.status, GitDiffStatus::Unchanged);
    assert_eq!(result.difference, 0.0);

    let _ = std::fs::remove_file(&tmp_a);
    let _ = std::fs::remove_file(&tmp_b);
}

#[tokio::test]
async fn get_diff_data_directory_path_returns_not_a_file() {
    let tmp_dir = std::env::temp_dir().join(format!("diff_test_dir_{}", std::process::id()));
    let _ = std::fs::create_dir_all(&tmp_dir);

    let manager = sut();
    let result = manager
        .get_diff_data(tmp_dir.to_str().unwrap(), tmp_dir.to_str().unwrap())
        .await;
    assert_eq!(result.status, GitDiffStatus::NotAFile);

    let _ = std::fs::remove_dir_all(&tmp_dir);
}
