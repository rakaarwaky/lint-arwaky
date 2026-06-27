use git_hooks_lint_arwaky::agent_git_hooks_orchestrator::GitHooksOrchestrator;
use shared::cli_commands::taxonomy_result_vo::LintResultList;
use shared::common::taxonomy_path_vo::FilePath;
use shared::git_hooks::contract_diff_protocol::IDiffProtocol;
use shared::git_hooks::contract_git_hooks_aggregate::GitHooksAggregate;
use shared::git_hooks::contract_hook_protocol::IHookProtocol;
use shared::git_hooks::contract_manager_port::IHookManagerPort;
use shared::git_hooks::contract_orchestrator_aggregate::HookManagementOrchestratorAggregate;
use shared::git_hooks::taxonomy_diff_result_vo::GitDiffResultVO;
use shared::git_hooks::taxonomy_git_diff_data_vo::GitDiffStatus;
use shared::git_hooks::taxonomy_hook_error::GitHookError;
use shared::mcp_server::taxonomy_job_vo::SuccessStatus;
use std::sync::Arc;

struct MockDiff;
struct MockHook;
struct MockManager;

#[async_trait::async_trait]
impl IDiffProtocol for MockDiff {
    async fn run_git_diff_check(&self, _path: &FilePath) -> LintResultList {
        LintResultList::default()
    }
    async fn get_diff(&self, _path: &FilePath) -> GitDiffResultVO {
        GitDiffResultVO::default()
    }
    async fn get_changed_files(&self, _path: &FilePath) -> shared::common::taxonomy_paths_vo::FilePathList {
        shared::common::taxonomy_paths_vo::FilePathList::default()
    }
    async fn get_default_branch(&self, _path: &FilePath) -> String {
        "main".to_string()
    }
}

#[async_trait::async_trait]
impl IHookProtocol for MockHook {
    async fn install_pre_commit(&self, _path: &FilePath) -> Result<SuccessStatus, GitHookError> {
        Ok(SuccessStatus::new(true))
    }
    async fn uninstall_pre_commit(&self) -> Result<SuccessStatus, GitHookError> {
        Ok(SuccessStatus::new(true))
    }
    fn get_hook_manager_identity(&self) -> shared::taxonomy_layer_vo::Identity {
        shared::taxonomy_layer_vo::Identity::new("mock_hook")
    }
    async fn initialize_config(&self, _path: &str) -> shared::taxonomy_suggestion_vo::DescriptionVO {
        shared::taxonomy_suggestion_vo::DescriptionVO::new("ok")
    }
    fn update_ignore_rule(&self, _request: shared::git_hooks::taxonomy_git_diff_data_vo::HookIgnoreUpdateVO) -> shared::taxonomy_suggestion_vo::DescriptionVO {
        shared::taxonomy_suggestion_vo::DescriptionVO::new("ok")
    }
    async fn get_diff_data(&self, _p1: &str, _p2: &str) -> shared::git_hooks::taxonomy_git_diff_data_vo::GitDiffDataVO {
        shared::git_hooks::taxonomy_git_diff_data_vo::GitDiffDataVO::default()
    }
}

impl IHookManagerPort for MockManager {
    fn install_pre_commit(&self, _path: &FilePath) -> Result<SuccessStatus, GitHookError> {
        Ok(SuccessStatus::new(true))
    }
    fn uninstall_pre_commit(&self) -> Result<SuccessStatus, GitHookError> {
        Ok(SuccessStatus::new(true))
    }
}

fn make_orchestrator() -> GitHooksOrchestrator {
    GitHooksOrchestrator::new(
        Arc::new(MockDiff),
        Arc::new(MockHook),
        Arc::new(MockManager),
    )
}

#[test]
fn orchestrator_can_be_constructed() {
    let _ = make_orchestrator();
}

#[test]
fn diff_protocol_returns_mock() {
    let orch = make_orchestrator();
    let _dp = orch.diff_protocol();
}

#[test]
fn hook_protocol_returns_mock() {
    let orch = make_orchestrator();
    let _hp = orch.hook_protocol();
}

#[test]
fn get_hook_manager_identity_delegates() {
    let orch = make_orchestrator();
    let id = orch.get_hook_manager_identity();
    assert_eq!(id.value(), "mock_hook");
}

#[tokio::test]
async fn run_git_hooks_check_returns_empty() {
    let orch = make_orchestrator();
    let path = FilePath::new(".".to_string()).unwrap_or_default();
    let result = orch.run_git_hooks_check(&path).await;
    assert!(result.values.is_empty());
}

#[tokio::test]
async fn install_hook_returns_ok() {
    let orch = make_orchestrator();
    let path = FilePath::new("/tmp/test".to_string()).unwrap_or_default();
    let result = orch.install_hook(&path).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn uninstall_hook_returns_ok() {
    let orch = make_orchestrator();
    let result = orch.uninstall_hook().await;
    assert!(result.is_ok());
}

#[test]
fn get_hook_manager_returns_manager() {
    let orch = make_orchestrator();
    let manager = orch.get_hook_manager();
    let result = manager.install_pre_commit(
        &FilePath::new("test".to_string()).unwrap_or_default(),
    );
    assert!(result.is_ok());
}
