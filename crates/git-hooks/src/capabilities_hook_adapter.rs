// PURPOSE: GitHookAdapter — FR-002/FR-003 protocol implementations (capabilities layer)
//
// Handles .git/hooks/ directory creation, hook script writing, permission
// setting, and hook removal. This is the lowest-level hook component that
// interacts directly with the filesystem.

use shared::common::taxonomy_job_vo::SuccessStatus;
use shared::common::taxonomy_layer_vo::Identity;
use shared::common::taxonomy_message_vo::LintMessage;
use shared::common::taxonomy_path_vo::FilePath;

use shared::filesystem::contract_filesystem_protocol::IFileSystemIOProtocol;
use shared::filesystem::taxonomy_filesystem_vo::FileMode;
use shared::git_hooks::contract_git_hooks_protocol::IHookInstallProtocol;
use shared::git_hooks::contract_git_hooks_protocol::IHookUninstallProtocol;
use shared::git_hooks::taxonomy_hook_error::GitHookError;
use std::sync::Arc;

// ─── Block 1: Struct Definition ───────────────────────────

pub struct GitHookAdapter {
    root_dir: FilePath,
    io: Arc<dyn IFileSystemIOProtocol>,
}

// ─── Block 2: Protocol Trait Implementation ───────────────

impl IHookInstallProtocol for GitHookAdapter {
    fn install_pre_commit(
        &self,
        executable_path: &FilePath,
    ) -> Result<SuccessStatus, GitHookError> {
        if !self.is_git_repo() {
            return Ok(SuccessStatus::new(false));
        }
        let hooks_dir = self.git_dir().join("hooks");
        self.io.create_dir_all(&hooks_dir).map_err(|e| {
            GitHookError::new(LintMessage::new(format!(
                "Failed to create hooks dir: {}",
                e
            )))
        })?;
        let hook_path = hooks_dir.join("pre-commit");
        let exe_str = if executable_path.value.is_empty() {
            "lint-arwaky-cli"
        } else {
            &executable_path.value
        };
        let hook_content = format!(
            "#!/bin/bash
# Lint Arwaky Pre-Commit Hook
echo \"Running Lint Arwaky check...\"
{} check .
if [ $? -ne 0 ]; then
  echo \"Linting failed. Please fix issues before committing.\"
  exit 1
fi
echo \"Linting passed.\"
exit 0
",
            exe_str
        );
        self.io
            .write_string(&hook_path, &hook_content)
            .map_err(|e| {
                GitHookError::new(LintMessage::new(format!("Failed to write hook: {}", e)))
            })?;
        #[cfg(unix)]
        {
            self.io
                .set_permissions(&hook_path, FileMode::new(0o755))
                .map_err(|e| {
                    GitHookError::new(LintMessage::new(format!(
                        "Failed to set permissions: {}",
                        e
                    )))
                })?;
        }
        Ok(SuccessStatus::new(true))
    }

    fn get_hook_manager_identity(&self) -> Identity {
        Identity::new("git_hook_manager")
    }
}

impl IHookUninstallProtocol for GitHookAdapter {
    fn uninstall_pre_commit(&self) -> Result<SuccessStatus, GitHookError> {
        if !self.is_git_repo() {
            return Ok(SuccessStatus::new(false));
        }
        let hook_path = self.git_dir().join("hooks").join("pre-commit");
        if self.io.path_exists(&hook_path) {
            self.io.remove_file(&hook_path).map_err(|e| {
                GitHookError::new(LintMessage::new(format!("Failed to remove hook: {}", e)))
            })?;
        }
        Ok(SuccessStatus::new(true))
    }
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────

impl GitHookAdapter {
    pub fn new(root_dir: FilePath, io: Arc<dyn IFileSystemIOProtocol>) -> Self {
        Self { root_dir, io }
    }

    fn git_dir(&self) -> std::path::PathBuf {
        std::path::PathBuf::from(&self.root_dir.value).join(".git")
    }

    fn is_git_repo(&self) -> bool {
        let git = self.git_dir();
        self.io.is_dir(&git)
    }
}
