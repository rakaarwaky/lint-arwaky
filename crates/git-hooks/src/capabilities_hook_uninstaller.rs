// PURPOSE: HookUninstaller — FR-003 protocol implementation (capabilities layer)
//
// Handles removal of the pre-commit hook script from `.git/hooks/`.
// Idempotent — returns success even if the hook does not exist.

use shared_common::taxonomy_job_vo::SuccessStatus;
use shared_common::taxonomy_message_vo::LintMessage;

use shared_filesystem::contract_filesystem_protocol::IFileSystemIOProtocol;
use shared_git_hooks::contract_git_hooks_protocol::IHookUninstallProtocol;
use shared_git_hooks::taxonomy_git_hooks_error::GitHookError;
use std::sync::Arc;

// ─── Block 1: Struct Definition ───────────────────────────

pub struct HookUninstaller {
    root_dir: String,
    io: Arc<dyn IFileSystemIOProtocol>,
}

// ─── Block 2: Protocol Trait Implementation ───────────────

impl IHookUninstallProtocol for HookUninstaller {
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

impl HookUninstaller {
    pub fn new(root_dir: String, io: Arc<dyn IFileSystemIOProtocol>) -> Self {
        Self { root_dir, io }
    }

    fn git_dir(&self) -> std::path::PathBuf {
        std::path::PathBuf::from(&self.root_dir).join(".git")
    }

    fn is_git_repo(&self) -> bool {
        let git = self.git_dir();
        self.io.is_dir(&git)
    }
}
