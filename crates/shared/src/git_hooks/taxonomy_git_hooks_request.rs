// PURPOSE: GitHooksRequest — request payload for the git_hooks aggregate

use shared_common::taxonomy_path_vo::FilePath;

pub enum GitHooksRequest {
    /// Run the full staged-file check on a path.
    RunCheck { path: FilePath },
    /// Install the pre-commit hook.
    Install { executable_path: FilePath },
    /// Remove the pre-commit hook.
    Uninstall,
    /// Return the hook manager's identity.
    GetManagerIdentity,
}

impl GitHooksRequest {
    pub fn run_check(path: &FilePath) -> Self {
        Self::RunCheck { path: path.clone() }
    }

    pub fn install(executable_path: &FilePath) -> Self {
        Self::Install {
            executable_path: executable_path.clone(),
        }
    }

    pub fn uninstall() -> Self {
        Self::Uninstall
    }
}
