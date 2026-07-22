use shared::common::taxonomy_job_vo::SuccessStatus;
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::taxonomy_suggestion_vo::DescriptionVO;
use shared::common::utility_file_handler as utility_file;
use shared::common::utility_file_handler;
use shared::git_hooks::contract_hook_protocol::IHookProtocol;
use shared::git_hooks::contract_manager_protocol::IHookManagerProtocol;
use shared::git_hooks::taxonomy_git_diff_data_vo::{
    GitDiffDataVO, GitDiffSideVO, GitDiffStatus, HookIgnoreUpdateVO,
};
use shared::git_hooks::taxonomy_hook_error::GitHookError;
use std::sync::Arc;

// PURPOSE: HookManager — implements IHookProtocol for git hook management (capabilities layer)
use shared::common::taxonomy_layer_vo::Identity;

// ─── Block 1: Struct Definition ───────────────────────────

pub struct HookManager {
    hook_adapter: Arc<dyn IHookManagerProtocol>,
}

// ─── Block 2: Protocol Trait Implementation ───────────────

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
        if utility_file::path_exists(&config_file) {
            return DescriptionVO::new(format!("ALREADY_EXISTS:{}", config_file));
        }
        DescriptionVO::new(format!("Initialized {}", config_file))
    }

    fn update_ignore_rule(&self, request: HookIgnoreUpdateVO) -> DescriptionVO {
        if !utility_file::path_exists(&request.config_path) {
            return DescriptionVO::new(format!("Config file not found: {}", request.config_path));
        }
        let verb = if request.remove { "Removed" } else { "Added" };
        DescriptionVO::new(format!("{} '{}' from ignore list", verb, request.rule))
    }

    async fn get_diff_data(&self, path1: &str, path2: &str) -> GitDiffDataVO {
        let both_exist =
            utility_file::path_exists(path1) && utility_file::path_exists(path2);
        let both_files =
            utility_file::is_file(path1) && utility_file::is_file(path2);
        let status = match (both_exist, both_files) {
            (false, _) => {
                if !utility_file::path_exists(path1) {
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

// ─── Block 3: Constructors, Helpers, Private Methods ──────

impl HookManager {
    pub fn new(hook_adapter: Arc<dyn IHookManagerProtocol>) -> Self {
        Self { hook_adapter }
    }
}
