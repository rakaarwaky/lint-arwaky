// PURPOSE: HookInstaller — FR-002 protocol implementation (capabilities layer)
//
// Handles .git/hooks/ directory creation, hook script writing, and permission
// setting. This is the lowest-level install component that interacts directly
// with the filesystem.

use shared_common::taxonomy_job_vo::SuccessStatus;
use shared_common::taxonomy_layer_vo::Identity;
use shared_common::taxonomy_message_vo::LintMessage;
use shared_common::taxonomy_path_vo::FilePath;

use shared_filesystem::contract_filesystem_protocol::IFileSystemIOProtocol;
use shared_filesystem::taxonomy_filesystem_vo::FileMode;
use shared_git_hooks::contract_git_hooks_protocol::IHookInstallProtocol;
use shared_git_hooks::taxonomy_git_hooks_error::GitHookError;
use std::sync::Arc;

// ─── Block 1: Struct Definition ───────────────────────────

pub struct HookInstaller {
    root_dir: FilePath,
    io: Arc<dyn IFileSystemIOProtocol>,
}

// ─── Block 2: Protocol Trait Implementation ───────────────

impl IHookInstallProtocol for HookInstaller {
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
        const MANAGED_MARKER: &str = "# managed-by: lint-arwaky";
        if self.io.path_exists(&hook_path) {
            let existing = self.io.read_to_string(&hook_path).map_err(|e| {
                GitHookError::new(LintMessage::new(format!("Failed to inspect existing hook: {e}")))
            })?;
            if !existing.value.contains(MANAGED_MARKER) {
                let backup_path = hooks_dir.join("pre-commit.bak");
                self.io.copy_file(&hook_path, &backup_path).map_err(|e| {
                    GitHookError::new(LintMessage::new(format!(
                        "Existing pre-commit hook is unmanaged and could not be backed up to {}: {e}",
                        backup_path.display()
                    )))
                })?;
            }
        }
        let exe_str = if executable_path.value.is_empty() {
            "lint-arwaky-cli"
        } else {
            &executable_path.value
        };
        let hook_content = format!(
            "#!/bin/bash
# managed-by: lint-arwaky
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

// ─── Block 3: Constructors, Helpers, Private Methods ──────

impl HookInstaller {
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
