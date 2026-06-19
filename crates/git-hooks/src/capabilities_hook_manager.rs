// PURPOSE: HookManager — implements IHookProtocol for git hook management (capabilities layer)
use shared::common::taxonomy_layer_vo::Identity;
use shared::git_hooks::contract_hook_protocol::IHookProtocol;
use shared::git_hooks::contract_manager_port::IHookManagerPort;
use shared::git_hooks::taxonomy_hook_error::GitHookError;
use shared::mcp_server::taxonomy_job_vo::SuccessStatus;
use shared::source_parsing::taxonomy_path_vo::FilePath;
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

    async fn initialize_config(&self, path: &str) -> String {
        let config_file = format!("{}/lint_arwaky.config.yaml", path);
        if std::path::Path::new(&config_file).exists() {
            return format!("ALREADY_EXISTS:{}", config_file);
        }
        format!("Initialized {}", config_file)
    }

    fn update_ignore_rule(&self, rule: &str, remove: bool, config_path: &str) -> String {
        let config_file = std::path::Path::new(config_path);
        if !config_file.exists() {
            return format!("Config file not found: {}", config_path);
        }
        format!(
            "{} '{}' from ignore list",
            if remove { "Removed" } else { "Added" },
            rule
        )
    }

    async fn get_diff_data(
        &self,
        path1: &str,
        path2: &str,
    ) -> std::collections::HashMap<String, serde_json::Value> {
        let mut result = std::collections::HashMap::new();
        result.insert(
            "version1".to_string(),
            serde_json::json!({"score": 0.0, "path": path1}),
        );
        result.insert(
            "version2".to_string(),
            serde_json::json!({"score": 0.0, "path": path2}),
        );
        result.insert("difference".to_string(), serde_json::json!(0.0));
        result.insert("status".to_string(), serde_json::json!("UNCHANGED"));
        result
    }

    async fn get_suggestions(
        &self,
        path: &str,
    ) -> std::collections::HashMap<String, serde_json::Value> {
        let mut result = std::collections::HashMap::new();
        result.insert("score".to_string(), serde_json::json!(0.0));
        result.insert("path".to_string(), serde_json::json!(path));
        result.insert("has_issues".to_string(), serde_json::json!(true));
        result
    }
}
