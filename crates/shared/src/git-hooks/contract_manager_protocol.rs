// PURPOSE: IHookManagerProtocol — protocol trait for hook script management (install, uninstall)

use crate::common::taxonomy_path_vo::FilePath;
use crate::git_hooks::taxonomy_hook_error::GitHookError;
use crate::common::taxonomy_job_vo::SuccessStatus;

pub trait IHookManagerProtocol: Send + Sync {
    fn install_pre_commit(&self, executable_path: &FilePath)
        -> Result<SuccessStatus, GitHookError>;
    fn uninstall_pre_commit(&self) -> Result<SuccessStatus, GitHookError>;
}
