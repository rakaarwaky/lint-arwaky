// PURPOSE: GitHooksRequest/GitHooksResponse — request/response VOs for the git-hooks aggregate
use crate::common::taxonomy_job_vo::SuccessStatus;
use crate::common::taxonomy_lint_result_vo::LintResultList;
use crate::common::taxonomy_layer_vo::Identity;
use crate::common::taxonomy_path_vo::FilePath;
use crate::common::taxonomy_suggestion_vo::DescriptionVO;
use crate::git_hooks::taxonomy_git_diff_data_vo::{GitDiffDataVO, HookIgnoreUpdateVO};
use crate::git_hooks::taxonomy_hook_error::GitHookError;

/// Consumer verb carried by the git-hooks aggregate's single entry point.
pub enum GitHooksRequest {
    /// Run the full staged-file check on a path.
    RunCheck { path: FilePath },
    /// Install the pre-commit hook.
    Install { executable_path: FilePath },
    /// Remove the pre-commit hook.
    Uninstall,
    /// Write the lint config at a project path.
    InitializeConfig { path: String },
    /// Add or remove an ignore rule in config.
    UpdateIgnoreRule { request: HookIgnoreUpdateVO },
    /// Compare two file paths for diff data.
    DiffData { path1: String, path2: String },
    /// Return the hook manager's identity.
    GetManagerIdentity,
}

impl GitHooksRequest {
    pub fn run_check(path: &FilePath) -> Self {
        Self::RunCheck { path: path.clone() }
    }

    pub fn install(executable_path: &FilePath) -> Self {
        Self::Install { executable_path: executable_path.clone() }
    }

    pub fn uninstall() -> Self {
        Self::Uninstall
    }

    pub fn initialize_config(path: &str) -> Self {
        Self::InitializeConfig { path: path.to_string() }
    }

    pub fn update_ignore_rule(request: HookIgnoreUpdateVO) -> Self {
        Self::UpdateIgnoreRule { request }
    }

    pub fn diff_data(path1: &str, path2: &str) -> Self {
        Self::DiffData { path1: path1.to_string(), path2: path2.to_string() }
    }

    pub fn get_manager_identity() -> Self {
        Self::GetManagerIdentity
    }
}

/// Result of a git-hooks aggregate request.
pub enum GitHooksResponse {
    RunCheck { results: LintResultList },
    Install { status: Result<SuccessStatus, GitHookError> },
    Uninstall { status: Result<SuccessStatus, GitHookError> },
    InitializeConfig { description: DescriptionVO },
    UpdateIgnoreRule { description: DescriptionVO },
    DiffData { data: GitDiffDataVO },
    GetManagerIdentity { identity: Identity },
}

impl GitHooksResponse {
    pub fn into_results(self) -> LintResultList {
        match self {
            Self::RunCheck { results } => results,
            _ => panic!("expected RunCheck response"),
        }
    }

    pub fn into_status(self) -> Result<SuccessStatus, GitHookError> {
        match self {
            Self::Install { status } | Self::Uninstall { status } => status,
            _ => panic!("expected Install/Uninstall response"),
        }
    }

    pub fn into_description(self) -> DescriptionVO {
        match self {
            Self::InitializeConfig { description } | Self::UpdateIgnoreRule { description } => {
                description
            }
            _ => panic!("expected a config response"),
        }
    }

    pub fn into_diff_data(self) -> GitDiffDataVO {
        match self {
            Self::DiffData { data } => data,
            _ => panic!("expected DiffData response"),
        }
    }

    pub fn into_identity(self) -> Identity {
        match self {
            Self::GetManagerIdentity { identity } => identity,
            _ => panic!("expected GetManagerIdentity response"),
        }
    }
}
