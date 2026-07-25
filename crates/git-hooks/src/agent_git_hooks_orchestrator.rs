// PURPOSE: GitHooksOrchestrator — orchestrates git hooks operations by delegating to protocols/ports only (agent layer)
//
// The git hooks feature provides pre-commit enforcement: before each commit,
// lint-arwaky runs `check` on staged files. If violations are found, the
// commit is blocked.
//
// This orchestrator delegates to three sub-components:
//   - IDiffProtocol: extracts the diff of staged files (git diff --cached)
//   - IHookProtocol: manages hook lifecycle (install/uninstall the hook script)
//   - IHookManagerProtocol: low-level file operations for .git/hooks/ directory
//
// The orchestrator itself contains no git logic — it's pure composition.
use shared::cli_commands::LintResultList;
use shared::common::{
    SuccessStatus,
    FilePath,
};

use shared::git_hooks::{
    IDiffProtocol,
    GitHooksAggregate,
    IHookProtocol,
    IHookManagerProtocol,
    HookManagementOrchestratorAggregate,
    GitHookError,
};

use std::sync::Arc;

// ─── Block 1: Struct Definition ───────────────────────────
pub struct GitHooksOrchestrator {
    diff_protocol: Arc<dyn IDiffProtocol>,
    hook_protocol: Arc<dyn IHookProtocol>,
    hook_manager: Arc<dyn IHookManagerProtocol>,
}

// ─── Block 2: Aggregate Trait Implementation ──────────────
#[async_trait::async_trait]
impl GitHooksAggregate for GitHooksOrchestrator {
    fn diff_protocol(&self) -> &dyn IDiffProtocol {
        self.diff_protocol.as_ref()
    }

    fn hook_protocol(&self) -> &dyn IHookProtocol {
        self.hook_protocol.as_ref()
    }

    async fn run_git_hooks_check(&self, path: &FilePath) -> LintResultList {
        self.diff_protocol().run_git_diff_check(path).await
    }

    async fn install_hook(
        &self,
        executable_path: &FilePath,
    ) -> Result<SuccessStatus, GitHookError> {
        self.hook_protocol()
            .install_pre_commit(executable_path)
            .await
    }

    async fn uninstall_hook(&self) -> Result<SuccessStatus, GitHookError> {
        self.hook_protocol().uninstall_pre_commit().await
    }
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────
impl GitHooksOrchestrator {
    pub fn new(
        diff_protocol: Arc<dyn IDiffProtocol>,
        hook_protocol: Arc<dyn IHookProtocol>,
        hook_manager: Arc<dyn IHookManagerProtocol>,
    ) -> Self {
        Self {
            diff_protocol,
            hook_protocol,
            hook_manager,
        }
    }
}

#[async_trait::async_trait]
impl HookManagementOrchestratorAggregate for GitHooksOrchestrator {
    fn get_hook_manager(&self) -> &dyn IHookManagerProtocol {
        self.hook_manager.as_ref()
    }

    fn get_hook_manager_identity(&self) -> shared::taxonomy_layer_vo::Identity {
        self.hook_protocol().get_hook_manager_identity()
    }
}
