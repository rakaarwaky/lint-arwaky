use git_hooks_lint_arwaky::capabilities_hook_manager::HookManager;
use shared::common::taxonomy_path_vo::FilePath;
use shared::git_hooks::contract_git_file_check_port::IGitFileCheckPort;
use shared::git_hooks::contract_hook_protocol::IHookProtocol;
use shared::git_hooks::contract_manager_port::IHookManagerPort;
use shared::git_hooks::taxonomy_git_diff_data_vo::{GitDiffStatus, HookIgnoreUpdateVO};
use shared::git_hooks::taxonomy_hook_error::GitHookError;
use shared::mcp_server::taxonomy_job_vo::SuccessStatus;
use std::sync::Arc;

struct MockAdapter;

impl IHookManagerPort for MockAdapter {
    fn install_pre_commit(
        &self,
        _executable_path: &FilePath,
    ) -> Result<SuccessStatus, GitHookError> {
        Ok(SuccessStatus::new(true))
    }
    fn uninstall_pre_commit(&self) -> Result<SuccessStatus, GitHookError> {
        Ok(SuccessStatus::new(true))
    }
}

struct MockFileCheck;

#[async_trait::async_trait]
impl IGitFileCheckPort for MockFileCheck {
    async fn path_exists(&self, _path: &str) -> bool {
        false
    }
    async fn is_file(&self, _path: &str) -> bool {
        false
    }
    async fn is_dir(&self, _path: &str) -> bool {
        false
    }
}

fn make_manager() -> HookManager {
    HookManager::new(Arc::new(MockAdapter), Arc::new(MockFileCheck))
}

#[test]
fn get_hook_manager_identity_returns_fixed() {
    let manager = make_manager();
    let id = manager.get_hook_manager_identity();
    assert_eq!(id.value(), "git_hook_manager");
}

// ─── initialize_config ─────────────────────────────────────────────────────

#[tokio::test]
async fn initialize_config_returns_already_exists_for_existing_file() {
    let manager = make_manager();
    let dir = std::env::temp_dir().join(format!("hook_init_{}", std::process::id()));
    let _ = std::fs::create_dir_all(&dir);
    std::fs::write(dir.join("lint_arwaky.config.yaml"), "dummy").unwrap();
    let result = manager.initialize_config(&dir.to_string_lossy()).await;
    assert!(result.value().contains("ALREADY_EXISTS"));
    let _ = std::fs::remove_dir_all(&dir);
}

#[tokio::test]
async fn initialize_config_returns_initialized_for_new_path() {
    let manager = make_manager();
    let dir = std::env::temp_dir().join(format!("hook_init_new_{}", std::process::id()));
    let _ = std::fs::create_dir_all(&dir);
    let result = manager.initialize_config(&dir.to_string_lossy()).await;
    assert!(result.value().contains("Initialized"));
    let _ = std::fs::remove_dir_all(&dir);
}

// ─── update_ignore_rule ────────────────────────────────────────────────────

#[test]
fn update_ignore_rule_adds_rule() {
    let manager = make_manager();
    let dir = std::env::temp_dir().join(format!("hook_ignore_{}", std::process::id()));
    let _ = std::fs::create_dir_all(&dir);
    let config_path = dir.join("lint_arwaky.config.yaml");
    std::fs::write(&config_path, "dummy").unwrap();

    let request = HookIgnoreUpdateVO {
        config_path: config_path.to_string_lossy().to_string(),
        rule: "AES101".to_string(),
        remove: false,
    };
    let result = manager.update_ignore_rule(request);
    assert!(result.value().contains("Added"));
    assert!(result.value().contains("AES101"));
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn update_ignore_rule_removes_rule() {
    let manager = make_manager();
    let dir = std::env::temp_dir().join(format!("hook_ignore_rem_{}", std::process::id()));
    let _ = std::fs::create_dir_all(&dir);
    let config_path = dir.join("lint_arwaky.config.yaml");
    std::fs::write(&config_path, "dummy").unwrap();

    let request = HookIgnoreUpdateVO {
        config_path: config_path.to_string_lossy().to_string(),
        rule: "AES304".to_string(),
        remove: true,
    };
    let result = manager.update_ignore_rule(request);
    assert!(result.value().contains("Removed"));
    assert!(result.value().contains("AES304"));
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn update_ignore_rule_missing_config_returns_error() {
    let manager = make_manager();
    let request = HookIgnoreUpdateVO {
        config_path: "/nonexistent/path/config.yaml".to_string(),
        rule: "AES101".to_string(),
        remove: false,
    };
    let result = manager.update_ignore_rule(request);
    assert!(result.value().contains("Config file not found"));
}

// ─── get_diff_data ─────────────────────────────────────────────────────────

#[tokio::test]
async fn get_diff_data_both_missing_returns_missing_first() {
    let manager = make_manager();
    let result = manager
        .get_diff_data("/nonexistent/a.rs", "/nonexistent/b.rs")
        .await;
    assert_eq!(result.status, GitDiffStatus::MissingFirst);
}

#[tokio::test]
async fn get_diff_data_both_exist_files_returns_unchanged() {
    let manager = make_manager();
    let dir = std::env::temp_dir().join(format!("hook_diff_both_{}", std::process::id()));
    let _ = std::fs::create_dir_all(&dir);
    let a_path = dir.join("a.rs");
    let b_path = dir.join("b.rs");
    std::fs::write(&a_path, "content").unwrap();
    std::fs::write(&b_path, "content").unwrap();
    let result = manager
        .get_diff_data(&a_path.to_string_lossy(), &b_path.to_string_lossy())
        .await;
    assert_eq!(result.status, GitDiffStatus::Unchanged);
    let _ = std::fs::remove_dir_all(&dir);
}
