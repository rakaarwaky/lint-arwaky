// PURPOSE: IHookManagerProtocol — protocol trait for hook script management (install, uninstall)
//
// NOTE: This protocol is intentionally exposed from `git_hooks` to `mcp-server`
// because `mcp-server` needs it to construct the git container (composition root).
// This is the correct ownership boundary — `git_hooks` owns the protocol and
// `mcp-server` consumes it via composition root wiring.

use crate::common::taxonomy_job_vo::SuccessStatus;
use crate::common::taxonomy_path_vo::FilePath;
use crate::git_hooks::taxonomy_hook_error::GitHookError;

pub trait IHookManagerProtocol: Send + Sync {
    fn install_pre_commit(&self, executable_path: &FilePath)
    -> Result<SuccessStatus, GitHookError>;
    fn uninstall_pre_commit(&self) -> Result<SuccessStatus, GitHookError>;
}
