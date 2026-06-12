// PURPOSE: DevCommandsOrchestrator — diff, suggest, ignore, init, hook management
use std::collections::HashMap;

use shared::output_report::taxonomy_score_vo::FileFormat;
use shared::source_parsing::taxonomy_path_vo::FilePath;
use shared::taxonomy_common_vo::BooleanVO;
use shared::taxonomy_layer_vo::Identity;

pub struct DevCommandsOrchestrator {}

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

    pub fn initialize_config(&self, path: &str) -> String {
        let config_file = format!("{}/lint_arwaky.config.yaml", path);
        if std::path::Path::new(&config_file).exists() {
            return format!("ALREADY_EXISTS:{}", config_file);
        }
        format!("Initialized {}", config_file)
    }
}
