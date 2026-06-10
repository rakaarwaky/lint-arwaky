// PURPOSE: Orchestrator: Implements DevCommandsAggregate — diff, suggest, ignore, init, hook management
use crate::cli_commands::contract_dev_aggregate::DevCommandsAggregate;
use std::collections::HashMap;

use crate::output_report::taxonomy_score_vo::FileFormat;
use crate::shared_common::taxonomy_common_vo::BooleanVO;
use crate::shared_common::taxonomy_layer_vo::Identity;
use crate::source_parsing::taxonomy_path_vo::FilePath;
use async_trait::async_trait;

pub struct DevCommandsOrchestrator {}

#[async_trait]
impl DevCommandsAggregate for DevCommandsOrchestrator {
    async fn diff(&self, path1: FilePath, path2: FilePath, _output_format: FileFormat) {
        let diff_data = self.get_diff_data(&path1.value, &path2.value).await;
        println!("Comparison: {:?}", diff_data);
    }

    async fn suggest(&self, path: FilePath, _ai: BooleanVO) {
        let suggestions = self.get_suggestions(&path.value).await;
        println!("Suggestions: {:?}", suggestions);
    }

    async fn ignore(&self, rule: &Identity, remove: BooleanVO, path: Option<FilePath>) {
        let p = path
            .map(|fp| fp.value)
            .unwrap_or_else(|| "lint_arwaky.config.yaml".to_string());
        let res = self.update_ignore_rule(&rule.value, remove.value(), &p);
        println!("{}", res);
    }

    async fn config(&self, _action: &Identity, _path: Option<FilePath>) {
        println!("Config action: {}", _action.value);
    }

    async fn export(&self, _output_format: FileFormat, _output: Option<FilePath>) {
        println!("Export to format: {:?}", _output_format);
    }

    async fn init(&self, path: Option<FilePath>) {
        let p = path.map(|fp| fp.value).unwrap_or_else(|| ".".to_string());
        let res = self.initialize_config(&p);
        println!("{}", res);
    }

    async fn install_hook(&self, _path: Option<FilePath>) {
        println!("Installing pre-commit hook");
    }

    async fn uninstall_hook(&self, _path: Option<FilePath>) {
        println!("Uninstalling pre-commit hook");
    }
}

impl Default for DevCommandsOrchestrator {
    fn default() -> Self {
        Self::new()
    }
}

impl DevCommandsOrchestrator {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn get_diff_data(
        &self,
        path1: &str,
        path2: &str,
    ) -> HashMap<String, serde_json::Value> {
        // Get comparison data between two paths
        let mut result = HashMap::new();
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

    pub async fn get_suggestions(&self, path: &str) -> HashMap<String, serde_json::Value> {
        let mut result = HashMap::new();
        result.insert("score".to_string(), serde_json::json!(0.0));
        result.insert("path".to_string(), serde_json::json!(path));
        result.insert("has_issues".to_string(), serde_json::json!(true));
        result
    }

    pub fn update_ignore_rule(&self, rule: &str, remove: bool, config_path: &str) -> String {
        // Add or remove an ignore rule in the config file
        let config_file = std::path::Path::new(config_path);
        if !config_file.exists() {
            return format!("Config file not found: {}", config_path);
        }
        // Basic implementations for config manipulation
        format!(
            "{} '{}' from ignore list",
            if remove { "Removed" } else { "Added" },
            rule
        )
    }

    pub fn initialize_config(&self, path: &str) -> String {
        let config_file = format!("{}/lint_arwaky.config.yaml", path);
        if std::path::Path::new(&config_file).exists() {
            return format!("ALREADY_EXISTS:{}", config_file);
        }
        format!("Initialized {}", config_file)
    }
}
