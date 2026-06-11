// PURPOSE: IHookManagerPort — port trait for hook script management (install, uninstall)

use git_hooks::taxonomy_hook_error::GitHookError;
use pipeline_jobs::taxonomy_job_vo::SuccessStatus;
use source_parsing::taxonomy_path_vo::FilePath;

pub trait IHookManagerPort: Send + Sync {
    fn install_pre_commit(&self, executable_path: &FilePath)
        -> Result<SuccessStatus, GitHookError>;
    fn uninstall_pre_commit(&self) -> Result<SuccessStatus, GitHookError>;
}
