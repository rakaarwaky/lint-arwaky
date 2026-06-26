use git_hooks_lint_arwaky::capabilities_hook_manager::HookManager;
use shared::common::taxonomy_path_vo::FilePath;
use shared::git_hooks::contract_hook_protocol::IHookProtocol;
use shared::git_hooks::contract_manager_port::IHookManagerPort;
use shared::git_hooks::taxonomy_hook_error::GitHookError;
use shared::mcp_server::taxonomy_job_vo::SuccessStatus;
use std::sync::Arc;

struct MockAdapter;

impl IHookManagerPort for MockAdapter {
    fn install_pre_commit(
        &self, _executable_path: &FilePath,
    ) -> Result<SuccessStatus, GitHookError> {
        Ok(SuccessStatus::new(true))
    }
    fn uninstall_pre_commit(&self) -> Result<SuccessStatus, GitHookError> {
        Ok(SuccessStatus::new(true))
    }
}

#[test]
fn get_hook_manager_identity_returns_fixed() {
    let manager = HookManager::new(Arc::new(MockAdapter));
    let id = manager.get_hook_manager_identity();
    assert_eq!(id.value(), "git_hook_manager");
}
