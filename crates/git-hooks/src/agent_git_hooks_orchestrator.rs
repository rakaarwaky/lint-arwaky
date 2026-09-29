// PURPOSE: GitHooksOrchestrator — orchestrates git hooks operations by delegating to protocols only (agent layer)
//
// The git hooks feature provides pre-commit enforcement: before each commit,
// lint-arwaky runs `check` on staged files. If violations are found, the
// commit is blocked.
//
// The orchestrator composes one seam per FR (seven capability protocols) and
// holds no git logic of its own — it is pure composition.

use shared::cli_commands::LintResultList;
use shared::common::taxonomy_job_vo::SuccessStatus;
use shared::common::taxonomy_layer_vo::Identity;
use shared::common::taxonomy_path_vo::FilePath;
use shared::git_hooks::contract_git_hooks_aggregate::IGitHooksAggregate;
use shared::git_hooks::contract_git_hooks_protocol::IConfigInitProtocol;
use shared::git_hooks::contract_git_hooks_protocol::IDiffDataProtocol;
use shared::git_hooks::contract_git_hooks_protocol::IDiffDetectionProtocol;
use shared::git_hooks::contract_git_hooks_protocol::IHookCheckProtocol;
use shared::git_hooks::contract_git_hooks_protocol::IHookInstallProtocol;
use shared::git_hooks::contract_git_hooks_protocol::IHookUninstallProtocol;
use shared::git_hooks::contract_git_hooks_protocol::IIgnoreRuleProtocol;
use shared::git_hooks::taxonomy_git_hooks_error::GitHookError;
use shared::git_hooks::taxonomy_git_hooks_request::GitHooksRequest;
use shared::git_hooks::taxonomy_git_hooks_response::GitHooksResponse;
use shared::git_hooks::taxonomy_git_hooks_vo::GitDiffDataVO;
use shared::git_hooks::taxonomy_git_hooks_vo::HookIgnoreUpdateVO;

use std::sync::Arc;

// ─── Block 1: Struct Definition ───────────────────────────

pub struct GitHooksOrchestrator {
    diff_detection: Arc<dyn IDiffDetectionProtocol>,
    hook_check: Arc<dyn IHookCheckProtocol>,
    hook_install: Arc<dyn IHookInstallProtocol>,
    hook_uninstall: Arc<dyn IHookUninstallProtocol>,
    diff_data: Arc<dyn IDiffDataProtocol>,
    ignore_rules: Arc<dyn IIgnoreRuleProtocol>,
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
            GitHooksRequest::InitializeConfig { path } => GitHooksResponse::InitializeConfig {
                description: self.initialize_config(&path),
            },
            GitHooksRequest::UpdateIgnoreRule { request } => GitHooksResponse::UpdateIgnoreRule {
                description: self.update_ignore_rule(request),
            },
            GitHooksRequest::DiffData { path1, path2 } => GitHooksResponse::DiffData {
                data: self.get_diff_data(&path1, &path2),
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

    pub fn run_git_hooks_check(&self, path: &FilePath) -> LintResultList {
        self.hook_check.run_git_diff_check(path)
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
    ) -> shared::common::taxonomy_suggestion_vo::DescriptionVO {
        self.config_init.initialize_config(path)
    }

    pub fn update_ignore_rule(
        &self,
        request: HookIgnoreUpdateVO,
    ) -> shared::common::taxonomy_suggestion_vo::DescriptionVO {
        self.ignore_rules.update_ignore_rule(request)
    }

    pub fn get_diff_data(&self, path1: &str, path2: &str) -> GitDiffDataVO {
        self.diff_data.get_diff_data(path1, path2)
    }

    pub fn get_hook_manager_identity(&self) -> Identity {
        self.hook_install.get_hook_manager_identity()
    }

    pub fn new(
        diff_detection: Arc<dyn IDiffDetectionProtocol>,
        hook_check: Arc<dyn IHookCheckProtocol>,
        hook_install: Arc<dyn IHookInstallProtocol>,
        hook_uninstall: Arc<dyn IHookUninstallProtocol>,
        diff_data: Arc<dyn IDiffDataProtocol>,
        ignore_rules: Arc<dyn IIgnoreRuleProtocol>,
        config_init: Arc<dyn IConfigInitProtocol>,
    ) -> Self {
        Self {
            diff_detection,
            hook_check,
            hook_install,
            hook_uninstall,
            diff_data,
            ignore_rules,
            config_init,
        }
    }
}
