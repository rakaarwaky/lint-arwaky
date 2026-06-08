/// git_hook_adapter — Infrastructure adapter for Git hook management.
use crate::git_hooks::contract_manager_port::IHookManagerPort;
use crate::source_parsing::taxonomy_path_vo::FilePath;
use /* UNKNOWN: SuccessStatus */ crate::pipeline_jobs::taxonomy_job_vo::SuccessStatus;
use std::path::Path;

#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

pub struct GitHookAdapter {
    root_dir: FilePath,
}

impl GitHookAdapter {
    pub fn new(root_dir: FilePath) -> Self {
        Self { root_dir }
    }

    fn git_dir(&self) -> std::path::PathBuf {
        Path::new(&self.root_dir.value).join(".git")
    }

    fn is_git_repo(&self) -> bool {
        let git = self.git_dir();
        git.exists() && git.is_dir()
    }
}

impl IHookManagerPort for GitHookAdapter {
    fn install_pre_commit(
        &self,
        executable_path: &FilePath,
    ) -> Result<SuccessStatus, crate::git_hooks::taxonomy_hook_error::GitHookError> {
        if !self.is_git_repo() {
            return Ok(SuccessStatus::new(false));
        }
        let hooks_dir = self.git_dir().join("hooks");
        let _ = std::fs::create_dir_all(&hooks_dir);
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
        std::fs::write(&hook_path, &hook_content).map_err(|e| {
            crate::git_hooks::taxonomy_hook_error::GitHookError::new(crate::shared_common::taxonomy_common_error::ErrorMessage::new(format!(
                "Failed to write hook: {}",
                e
            )))
        })?;
        #[cfg(unix)]
        {
            let mut perms = std::fs::metadata(&hook_path)
                .map_err(|e| {
                    crate::git_hooks::taxonomy_hook_error::GitHookError::new(crate::shared_common::taxonomy_common_error::ErrorMessage::new(format!(
                        "Failed to get metadata: {}",
                        e
                    )))
                })?
                .permissions();
            perms.set_mode(0o755);
            std::fs::set_permissions(&hook_path, perms).map_err(|e| {
                crate::git_hooks::taxonomy_hook_error::GitHookError::new(crate::shared_common::taxonomy_common_error::ErrorMessage::new(format!(
                    "Failed to set permissions: {}",
                    e
                )))
            })?;
        }
        Ok(SuccessStatus::new(true))
    }

    fn uninstall_pre_commit(&self) -> Result<SuccessStatus, crate::git_hooks::taxonomy_hook_error::GitHookError> {
        if !self.is_git_repo() {
            return Ok(SuccessStatus::new(false));
        }
        let hook_path = self.git_dir().join("hooks").join("pre-commit");
        if hook_path.exists() {
            std::fs::remove_file(&hook_path).map_err(|e| {
                crate::git_hooks::taxonomy_hook_error::GitHookError::new(crate::shared_common::taxonomy_common_error::ErrorMessage::new(format!(
                    "Failed to remove hook: {}",
                    e
                )))
            })?;
        }
        Ok(SuccessStatus::new(true))
    }
}
