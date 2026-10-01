// PURPOSE: GitHooksOrchestrator — orchestrates git hooks operations by delegating to protocols only (agent layer)
//
// The git hooks feature provides pre-commit enforcement: before each commit,
// lint-arwaky runs `check` on staged files. If violations are found, the
// commit is blocked.
//
// The orchestrator composes four protocol seams (one per FR) and holds no
// logic of its own — it is pure composition.

use shared_common::taxonomy_job_vo::SuccessStatus;
use shared_common::taxonomy_layer_vo::Identity;
use shared_common::taxonomy_path_vo::FilePath;
use shared_git_hooks::contract_git_hooks_aggregate::IGitHooksAggregate;
use shared_git_hooks::contract_git_hooks_protocol::IConfigInitProtocol;
use shared_git_hooks::contract_git_hooks_protocol::IDiffDetectionProtocol;
use shared_git_hooks::contract_git_hooks_protocol::IHookInstallProtocol;
use shared_git_hooks::contract_git_hooks_protocol::IHookUninstallProtocol;
use shared_git_hooks::taxonomy_git_hooks_error::GitHookError;
use shared_git_hooks::taxonomy_git_hooks_request::GitHooksRequest;
use shared_git_hooks::taxonomy_git_hooks_response::GitHooksResponse;

use std::sync::Arc;

// ─── Block 1: Struct Definition ───────────────────────────

pub struct GitHooksOrchestrator {
    diff_detection: Arc<dyn IDiffDetectionProtocol>,
    hook_install: Arc<dyn IHookInstallProtocol>,
    hook_uninstall: Arc<dyn IHookUninstallProtocol>,
    config_init: Arc<dyn IConfigInitProtocol>,
}

// ─── Block 2: Aggregate Trait Implementations ─────────────

impl IGitHooksAggregate for GitHooksOrchestrator {
    fn execute(&self, request: GitHooksRequest) -> GitHooksResponse {
        match request {
            GitHooksRequest::RunCheck { path } => GitHooksResponse::RunCheck {
                results: self.run_git_hooks_check(&path),
            },
            GitHooksRequest::Install { executable_path } => GitHooksResponse::Install {
                status: self.install_hook(&executable_path),
            },
            GitHooksRequest::Uninstall => GitHooksResponse::Uninstall {
                status: self.uninstall_hook(),
            },
            GitHooksRequest::GetManagerIdentity => GitHooksResponse::GetManagerIdentity {
                identity: self.get_hook_manager_identity(),
            },
        }
    }
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────

impl GitHooksOrchestrator {
    pub fn diff_protocol(&self) -> &dyn IDiffDetectionProtocol {
        self.diff_detection.as_ref()
    }

    pub fn hook_protocol(&self) -> &dyn IHookInstallProtocol {
        self.hook_install.as_ref()
    }

    pub fn run_git_hooks_check(&self, path: &FilePath) -> shared_cli_commands::LintResultList {
        self.diff_detection.run_git_diff_check(path)
    }

    pub fn install_hook(&self, executable_path: &FilePath) -> Result<SuccessStatus, GitHookError> {
        self.hook_install.install_pre_commit(executable_path)
    }

    pub fn uninstall_hook(&self) -> Result<SuccessStatus, GitHookError> {
        self.hook_uninstall.uninstall_pre_commit()
    }

    pub fn initialize_config(
        &self,
        path: &str,
    ) -> shared_common::taxonomy_suggestion_vo::DescriptionVO {
        self.config_init.initialize_config(path)
    }

    pub fn update_ignore_rule(
        &self,
        request: shared_git_hooks::HookIgnoreUpdateVO,
    ) -> shared_common::taxonomy_suggestion_vo::DescriptionVO {
        self.config_init.update_ignore_rule(request)
    }

    pub fn get_hook_manager_identity(&self) -> Identity {
        self.hook_install.get_hook_manager_identity()
    }

    pub fn new(
        diff_detection: Arc<dyn IDiffDetectionProtocol>,
        hook_install: Arc<dyn IHookInstallProtocol>,
        hook_uninstall: Arc<dyn IHookUninstallProtocol>,
        config_init: Arc<dyn IConfigInitProtocol>,
    ) -> Self {
        Self {
            diff_detection,
            hook_install,
            hook_uninstall,
            config_init,
        }
    }
}
