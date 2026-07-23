use shared::common::taxonomy_job_vo::SuccessStatus;
use shared::common::taxonomy_path_vo::FilePath;
use shared::git_hooks::contract_manager_protocol::IHookManagerProtocol;
use shared::git_hooks::utility_git_io as git_io;

// PURPOSE: HookAdapter — IHookManagerProtocol implementation for installing/uninstalling git hook scripts

use shared::common::taxonomy_message_vo::LintMessage;

// ─── Block 1: Struct Definition ───────────────────────────

pub struct GitHookAdapter {
    root_dir: FilePath,
}

// ─── Block 2: Protocol Trait Implementation ───────────────

impl IHookManagerProtocol for GitHookAdapter {
    fn install_pre_commit(
        &self,
        executable_path: &FilePath,
    ) -> Result<SuccessStatus, shared::git_hooks::taxonomy_hook_error::GitHookError> {
        if !self.is_git_repo() {
            return Ok(SuccessStatus::new(false));
        }
        let hooks_dir = self.git_dir().join("hooks");
        git_io::create_dir_all(&hooks_dir).map_err(|e| {
            shared::git_hooks::taxonomy_hook_error::GitHookError::new(LintMessage::new(format!(
                "Failed to create hooks dir: {}",
                e
            )))
        })?;
        let hook_path = hooks_dir.join("pre-commit");
        let exe_str = if executable_path.value.is_empty() {
            "lint-arwaky"
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
        shared::common::utility_file_handler::write_file(&hook_path, &hook_content).map_err(
            |e| {
                shared::git_hooks::taxonomy_hook_error::GitHookError::new(LintMessage::new(
                    format!("Failed to write hook: {}", e),
                ))
            },
        )?;
        #[cfg(unix)]
        {
            git_io::set_permissions(&hook_path, 0o755).map_err(|e| {
                shared::git_hooks::taxonomy_hook_error::GitHookError::new(LintMessage::new(
                    format!("Failed to set permissions: {}", e),
                ))
            })?;
        }
        Ok(SuccessStatus::new(true))
    }

    fn uninstall_pre_commit(
        &self,
    ) -> Result<SuccessStatus, shared::git_hooks::taxonomy_hook_error::GitHookError> {
        if !self.is_git_repo() {
            return Ok(SuccessStatus::new(false));
        }
        let hook_path = self.git_dir().join("hooks").join("pre-commit");
        if shared::common::utility_file_handler::path_exists(&hook_path) {
            git_io::remove_file(&hook_path).map_err(|e| {
                shared::git_hooks::taxonomy_hook_error::GitHookError::new(LintMessage::new(
                    format!("Failed to remove hook: {}", e),
                ))
            })?;
        }
        Ok(SuccessStatus::new(true))
    }
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────

impl GitHookAdapter {
    pub fn new(root_dir: FilePath) -> Self {
        Self { root_dir }
    }

    fn git_dir(&self) -> std::path::PathBuf {
        std::path::PathBuf::from(&self.root_dir.value).join(".git")
    }

    fn is_git_repo(&self) -> bool {
        let git = self.git_dir();
        shared::common::utility_file_handler::is_dir(&git)
    }
}
