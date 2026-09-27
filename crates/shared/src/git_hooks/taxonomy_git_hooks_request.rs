// PURPOSE: GitHooksRequest — request payload for the git_hooks aggregate

use crate::common::taxonomy_path_vo::FilePath;
use crate::git_hooks::taxonomy_git_diff_data_vo::HookIgnoreUpdateVO;

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
        Self::Install {
            executable_path: executable_path.clone(),
        }
    }

    pub fn uninstall() -> Self {
        Self::Uninstall
    }

    pub fn initialize_config(path: &str) -> Self {
        Self::InitializeConfig {
            path: path.to_string(),
        }
    }

    pub fn update_ignore_rule(request: HookIgnoreUpdateVO) -> Self {
        Self::UpdateIgnoreRule { request }
    }

    pub fn diff_data(path1: &str, path2: &str) -> Self {
        Self::DiffData {
            path1: path1.to_string(),
            path2: path2.to_string(),
        }
    }

    pub fn get_manager_identity() -> Self {
        Self::GetManagerIdentity
    }
}
