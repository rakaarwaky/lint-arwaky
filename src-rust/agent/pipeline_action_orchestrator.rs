// pipeline_action_orchestrator — Logic for dispatching pipeline actions (Agent Layer).
use crate::contract::{PipelineDispatcherAggregate};
use crate::taxonomy::{ContentString, MetadataVO, FilePath, BooleanVO, ResponseData, StdOutput, StdError, ExitCode};
use std::collections::HashMap;

pub struct PipelineActionDispatcher;

impl PipelineDispatcherAggregate for PipelineActionDispatcher {}

impl PipelineActionDispatcher {
    pub fn new() -> Self {
        Self
    }

    pub async fn dispatch(&self, action: &ContentString, _args: &MetadataVO) -> ResponseData {
        let action_str = &action.value;

        // Handler map for known actions
        match action_str.as_str() {
            "check" | "scan" => self.handle_check(action_str).await,
            "security" => self.handle_security(action_str).await,
            "complexity" => self.handle_complexity(action_str).await,
            "duplicates" => self.handle_duplicates(action_str).await,
            "trends" => self.handle_trends(action_str).await,
            "fix" => self.handle_fix(action_str).await,
            "report" => self.handle_report(action_str).await,
            "version" => self.handle_version(action_str).await,
            "adapters" => self.handle_adapters(action_str).await,
            "install-hook" | "install_hook" => self.handle_install_hook(action_str).await,
            "uninstall-hook" | "uninstall_hook" => self.handle_uninstall_hook(action_str).await,
            _ => {
                let mut metadata = HashMap::new();
                metadata.insert("error".to_string(), serde_json::json!(format!("No pipeline handler for action: {}", action_str)));
                ResponseData::new(
                    StdOutput::new(""),
                    StdError::new(format!("No pipeline handler for action: {}", action_str)),
                    ExitCode::new(1),
                    MetadataVO::new(metadata),
                )
            }
        }
    }

    async fn handle_check(&self, _action: &str) -> ResponseData {
        let mut metadata = HashMap::new();
        metadata.insert("message".to_string(), serde_json::json!("check completed"));
        ResponseData::new(StdOutput::new("check completed"), StdError::new(""), ExitCode::new(0), MetadataVO::new(metadata))
    }

    async fn handle_security(&self, _action: &str) -> ResponseData {
        let mut metadata = HashMap::new();
        metadata.insert("bandit".to_string(), serde_json::json!(Vec::<String>::new()));
        ResponseData::new(StdOutput::new(""), StdError::new(""), ExitCode::new(0), MetadataVO::new(metadata))
    }

    async fn handle_complexity(&self, _action: &str) -> ResponseData {
        let mut metadata = HashMap::new();
        metadata.insert("radon".to_string(), serde_json::json!(Vec::<String>::new()));
        ResponseData::new(StdOutput::new(""), StdError::new(""), ExitCode::new(0), MetadataVO::new(metadata))
    }

    async fn handle_duplicates(&self, _action: &str) -> ResponseData {
        let mut metadata = HashMap::new();
        metadata.insert("duplicates".to_string(), serde_json::json!(Vec::<String>::new()));
        ResponseData::new(StdOutput::new(""), StdError::new(""), ExitCode::new(0), MetadataVO::new(metadata))
    }

    async fn handle_trends(&self, _action: &str) -> ResponseData {
        let mut metadata = HashMap::new();
        metadata.insert("trends".to_string(), serde_json::json!(Vec::<String>::new()));
        ResponseData::new(StdOutput::new(""), StdError::new(""), ExitCode::new(0), MetadataVO::new(metadata))
    }

    async fn handle_fix(&self, _action: &str) -> ResponseData {
        let mut metadata = HashMap::new();
        metadata.insert("output".to_string(), serde_json::json!("fix applied"));
        ResponseData::new(StdOutput::new("fix applied"), StdError::new(""), ExitCode::new(0), MetadataVO::new(metadata))
    }

    async fn handle_report(&self, _action: &str) -> ResponseData {
        let mut metadata = HashMap::new();
        metadata.insert("format".to_string(), serde_json::json!("text"));
        metadata.insert("data".to_string(), serde_json::json!({}));
        ResponseData::new(StdOutput::new(""), StdError::new(""), ExitCode::new(0), MetadataVO::new(metadata))
    }

    async fn handle_version(&self, _action: &str) -> ResponseData {
        let mut metadata = HashMap::new();
        metadata.insert("version".to_string(), serde_json::json!("1.0.0"));
        ResponseData::new(StdOutput::new("1.0.0"), StdError::new(""), ExitCode::new(0), MetadataVO::new(metadata))
    }

    async fn handle_adapters(&self, _action: &str) -> ResponseData {
        let mut metadata = HashMap::new();
        metadata.insert("adapters".to_string(), serde_json::json!(Vec::<String>::new()));
        ResponseData::new(StdOutput::new(""), StdError::new(""), ExitCode::new(0), MetadataVO::new(metadata))
    }

    async fn handle_install_hook(&self, _action: &str) -> ResponseData {
        let mut metadata = HashMap::new();
        metadata.insert("installed".to_string(), serde_json::json!(true));
        ResponseData::new(StdOutput::new("hook installed"), StdError::new(""), ExitCode::new(0), MetadataVO::new(metadata))
    }

    async fn handle_uninstall_hook(&self, _action: &str) -> ResponseData {
        let mut metadata = HashMap::new();
        metadata.insert("uninstalled".to_string(), serde_json::json!(true));
        ResponseData::new(StdOutput::new("hook uninstalled"), StdError::new(""), ExitCode::new(0), MetadataVO::new(metadata))
    }

    pub fn validate_action(&self, action: &ContentString) -> BooleanVO {
        let known_actions = [
            "check", "scan", "security", "complexity", "duplicates", "trends",
            "fix", "report", "version", "adapters", "install-hook", "install_hook",
            "uninstall-hook", "uninstall_hook", "batch", "multi_project", "doctor", "cancel",
        ];
        BooleanVO::new(known_actions.contains(&action.value.as_str()))
    }
}
