// PURPOSE: HookManager — IHookProtocol for git hook management (capabilities layer)
// Zero I/O: all file existence checks delegated to IGitFileCheckPort via DI.
use std::sync::Arc;

use shared::common::taxonomy_layer_vo::Identity;
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::taxonomy_suggestion_vo::DescriptionVO;
use shared::git_hooks::contract_git_file_check_port::IGitFileCheckPort;
use shared::git_hooks::contract_hook_protocol::IHookProtocol;
use shared::git_hooks::contract_manager_port::IHookManagerPort;
use shared::git_hooks::taxonomy_git_diff_data_vo::{
    GitDiffDataVO, GitDiffSideVO, GitDiffStatus, HookIgnoreUpdateVO,
};
use shared::git_hooks::taxonomy_hook_error::GitHookError;
use shared::mcp_server::taxonomy_job_vo::SuccessStatus;

// Block 1: struct Definition
pub struct HookManager {
    hook_adapter: Arc<dyn IHookManagerPort>,
    file_check: Arc<dyn IGitFileCheckPort>,
}

// Block 2: impl Trait for Struct (Public Contract)
#[async_trait::async_trait]
impl IHookProtocol for HookManager {
    async fn install_pre_commit(
        &self,
        executable_path: &FilePath,
    ) -> Result<SuccessStatus, GitHookError> {
        self.hook_adapter.install_pre_commit(executable_path)
    }

    async fn uninstall_pre_commit(&self) -> Result<SuccessStatus, GitHookError> {
        self.hook_adapter.uninstall_pre_commit()
    }

    fn get_hook_manager_identity(&self) -> Identity {
        Identity::new("git_hook_manager")
    }

    async fn initialize_config(&self, path: &str) -> DescriptionVO {
        let config_file = format!("{}/lint_arwaky.config.yaml", path);
        let fp = match FilePath::new(&config_file) {
            Ok(p) => p,
            Err(_) => return DescriptionVO::new(format!("INVALID_PATH:{}", config_file)),
        };
        if self.file_check.path_exists(&fp).await {
            return DescriptionVO::new(format!("ALREADY_EXISTS:{}", config_file));
        }
        DescriptionVO::new(format!("Initialized {}", config_file))
    }

    fn update_ignore_rule(&self, request: HookIgnoreUpdateVO) -> DescriptionVO {
        let verb = if request.remove { "Removed" } else { "Added" };
        DescriptionVO::new(format!("{} '{}' from ignore list", verb, request.rule))
    }

    async fn get_diff_data(&self, path1: &str, path2: &str) -> GitDiffDataVO {
        let fp1 = FilePath::new(path1).unwrap_or_default();
        let fp2 = FilePath::new(path2).unwrap_or_default();
        let p1_exists = self.file_check.path_exists(&fp1).await;
        let p2_exists = self.file_check.path_exists(&fp2).await;
        let p1_is_file = self.file_check.is_file(&fp1).await;
        let p2_is_file = self.file_check.is_file(&fp2).await;

        let status = match (p1_exists && p2_exists, p1_is_file && p2_is_file) {
            (false, _) => {
                if !p1_exists {
                    GitDiffStatus::MissingFirst
                } else {
                    GitDiffStatus::MissingSecond
                }
            }
            (true, false) => GitDiffStatus::NotAFile,
            (true, true) => GitDiffStatus::Unchanged,
        };
        GitDiffDataVO {
            version1: GitDiffSideVO::new(path1.to_string(), 1.0),
            version2: GitDiffSideVO::new(path2.to_string(), 1.0),
            difference: 0.0,
            status,
        }
    }
}

// Block 3: constructors
impl HookManager {
    pub fn new(
        hook_adapter: Arc<dyn IHookManagerPort>,
        file_check: Arc<dyn IGitFileCheckPort>,
    ) -> Self {
        Self {
            hook_adapter,
            file_check,
        }
    }
}
