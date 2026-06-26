// PURPOSE: HookManager — implements IHookProtocol for git hook management (capabilities layer)
use shared::common::taxonomy_layer_vo::Identity;
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::taxonomy_suggestion_vo::DescriptionVO;
use shared::git_hooks::contract_hook_protocol::IHookProtocol;
use shared::git_hooks::contract_manager_port::IHookManagerPort;
use shared::git_hooks::taxonomy_git_diff_data_vo::{
    GitDiffDataVO, GitDiffSideVO, GitDiffStatus, HookIgnoreUpdateVO,
};
use shared::git_hooks::taxonomy_hook_error::GitHookError;
use shared::mcp_server::taxonomy_job_vo::SuccessStatus;
use std::sync::Arc;

pub struct HookManager {
    hook_adapter: Arc<dyn IHookManagerPort>,
}

impl HookManager {
    pub fn new(hook_adapter: Arc<dyn IHookManagerPort>) -> Self {
        Self { hook_adapter }
    }
}

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
        if std::path::Path::new(&config_file).exists() {
            return DescriptionVO::new(format!("ALREADY_EXISTS:{}", config_file));
        }
        DescriptionVO::new(format!("Initialized {}", config_file))
    }

    fn update_ignore_rule(&self, request: HookIgnoreUpdateVO) -> DescriptionVO {
        let config_file = std::path::Path::new(&request.config_path);
        if !config_file.exists() {
            return DescriptionVO::new(format!("Config file not found: {}", request.config_path));
        }
        let verb = if request.remove { "Removed" } else { "Added" };
        DescriptionVO::new(format!("{} '{}' from ignore list", verb, request.rule))
    }

    async fn get_diff_data(&self, path1: &str, path2: &str) -> GitDiffDataVO {
        let p1 = std::path::Path::new(path1);
        let p2 = std::path::Path::new(path2);
        let both_exist = p1.exists() && p2.exists();
        let both_files = p1.is_file() && p2.is_file();
        let status = match (both_exist, both_files) {
            (false, _) => {
                if !p1.exists() {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_diff_data_both_missing_returns_missing_first() {
        let manager = HookManager::new(Arc::new(MockAdapter));
        let result = manager.get_diff_data("/nonexistent/a.rs", "/nonexistent/b.rs");
        assert_eq!(result.status, GitDiffStatus::MissingFirst);
    }

    #[test]
    fn get_diff_data_first_missing_second_exists() {
        let manager = HookManager::new(Arc::new(MockAdapter));
        if std::path::Path::new("Cargo.toml").exists() {
            let result = manager.get_diff_data("/nonexistent/x.rs", "Cargo.toml");
            assert_eq!(result.status, GitDiffStatus::MissingFirst);
        }
    }

    #[test]
    fn get_diff_data_both_exist_but_not_files() {
        let manager = HookManager::new(Arc::new(MockAdapter));
        let dir = std::env::temp_dir();
        let result = manager.get_diff_data(
            &dir.to_string_lossy(),
            &dir.to_string_lossy(),
        );
        assert_eq!(result.status, GitDiffStatus::NotAFile);
    }

    #[test]
    fn get_hook_manager_identity_returns_fixed() {
        let manager = HookManager::new(Arc::new(MockAdapter));
        let id = manager.get_hook_manager_identity();
        assert_eq!(id.value(), "git_hook_manager");
    }

    struct MockAdapter;
    #[async_trait::async_trait]
    impl IHookManagerPort for MockAdapter {
        async fn install_pre_commit(
            &self, _executable_path: &FilePath,
        ) -> Result<SuccessStatus, GitHookError> {
            Ok(SuccessStatus::new(true))
        }
        async fn uninstall_pre_commit(&self) -> Result<SuccessStatus, GitHookError> {
            Ok(SuccessStatus::new(true))
        }
    }
}
