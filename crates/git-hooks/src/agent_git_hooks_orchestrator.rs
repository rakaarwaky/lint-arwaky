// PURPOSE: GitHooksOrchestrator — orchestrates git hooks operations by delegating to protocols only (agent layer)
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
use shared::common::taxonomy_job_vo::SuccessStatus;
use shared::common::taxonomy_layer_vo::Identity;
use shared::common::taxonomy_path_vo::FilePath;
use shared::git_hooks::contract_git_hooks_protocol::IDiffProtocol;
use shared::git_hooks::contract_git_hooks_aggregate::IGitHooksAggregate;
use shared::git_hooks::contract_git_hooks_protocol::IHookProtocol;
use shared::git_hooks::contract_git_hooks_protocol::IHookManagerProtocol;
use shared::git_hooks::taxonomy_git_hooks_request_vo::{GitHooksRequest, GitHooksResponse};
use shared::git_hooks::taxonomy_hook_error::GitHookError;

use std::sync::Arc;

// ─── Block 1: Struct Definition ───────────────────────────

pub struct GitHooksOrchestrator {
    diff_protocol: Arc<dyn IDiffProtocol>,
    hook_protocol: Arc<dyn IHookProtocol>,
    hook_manager: Arc<dyn IHookManagerProtocol>,
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
    pub fn diff_protocol(&self) -> &dyn IDiffProtocol {
        self.diff_protocol.as_ref()
    }

    pub fn hook_protocol(&self) -> &dyn IHookProtocol {
        self.hook_protocol.as_ref()
    }

    pub fn run_git_hooks_check(&self, path: &FilePath) -> LintResultList {
        self.diff_protocol().run_git_diff_check(path)
    }

    pub fn install_hook(&self, executable_path: &FilePath) -> Result<SuccessStatus, GitHookError> {
        self.hook_protocol().install_pre_commit(executable_path)
    }

    pub fn uninstall_hook(&self) -> Result<SuccessStatus, GitHookError> {
        self.hook_protocol().uninstall_pre_commit()
    }

    pub fn initialize_config(&self, path: &str) -> shared::common::taxonomy_suggestion_vo::DescriptionVO {
        self.hook_protocol().initialize_config(path)
    }

    pub fn update_ignore_rule(&self, request: shared::git_hooks::taxonomy_git_diff_data_vo::HookIgnoreUpdateVO) -> shared::common::taxonomy_suggestion_vo::DescriptionVO {
        self.hook_protocol().update_ignore_rule(request)
    }

    pub fn get_diff_data(&self, path1: &str, path2: &str) -> shared::git_hooks::taxonomy_git_diff_data_vo::GitDiffDataVO {
        self.hook_protocol().get_diff_data(path1, path2)
    }

    pub fn get_hook_manager(&self) -> Arc<dyn IHookManagerProtocol> {
        self.hook_manager.clone()
    }
    pub fn get_hook_manager_identity(&self) -> Identity {
        self.hook_protocol().get_hook_manager_identity()
    }
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
