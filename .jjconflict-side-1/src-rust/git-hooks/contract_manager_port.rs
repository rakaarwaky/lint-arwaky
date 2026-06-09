//! Port trait for managing Git hooks.
//!
//! Defines the outbound interface for installing and uninstalling
//! pre-commit hooks that run lint checks before commits.

use crate::git_hooks::taxonomy_hook_error::GitHookError;
use crate::pipeline_jobs::taxonomy_job_vo::SuccessStatus;
use crate::source_parsing::taxonomy_path_vo::FilePath;

pub trait IHookManagerPort: Send + Sync {
    fn install_pre_commit(&self, executable_path: &FilePath)
        -> Result<SuccessStatus, GitHookError>;
    fn uninstall_pre_commit(&self) -> Result<SuccessStatus, GitHookError>;
}
