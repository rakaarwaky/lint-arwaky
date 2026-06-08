//! Port trait for managing Git hooks.
//!
//! Defines the outbound interface for installing and uninstalling
//! pre-commit hooks that run lint checks before commits.

use crate::taxonomy::FilePath;
use crate::taxonomy::GitHookError;
use crate::taxonomy::SuccessStatus;

pub trait IHookManagerPort: Send + Sync {
    fn install_pre_commit(&self, executable_path: &FilePath)
        -> Result<SuccessStatus, GitHookError>;
    fn uninstall_pre_commit(&self) -> Result<SuccessStatus, GitHookError>;
}
