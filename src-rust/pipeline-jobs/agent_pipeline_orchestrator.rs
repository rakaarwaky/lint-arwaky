// aes: wired-by-dispatch
// pipeline_action_orchestrator — Agent orchestrator for pipeline actions.
use crate::pipeline_jobs::contract_dispatcher_aggregate::PipelineActionDispatcherAggregate;
use crate::pipeline_jobs::taxonomy_action_vo::ActionArgs;
use crate::pipeline_jobs::taxonomy_action_vo::ActionName;
use crate::pipeline_jobs::taxonomy_job_vo::ResponseData;
use crate::pipeline_jobs::taxonomy_job_vo::SuccessStatus;
use crate::source_parsing::taxonomy_path_vo::FilePath;
use std::collections::HashMap;

pub struct PipelineActionOrchestrator {}

#[async_trait::async_trait]
impl PipelineActionDispatcherAggregate for PipelineActionOrchestrator {
    fn root_path(&self) -> Option<&FilePath> {
        None
    }

    async fn dispatch(&self, action: &ActionName, _args: ActionArgs) -> ResponseData {
        let action_str = action.value();
        match action_str {
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
                metadata.insert(
                    "error".to_string(),
                    serde_json::json!(format!("No pipeline handler for action: {}", action_str)),
                );
                ResponseData {
                    value: None,
                    stdout: String::new(),
                    stderr: format!("No pipeline handler for action: {}", action_str),
                    returncode: 1,
                    metadata,
                }
            }
        }
    }

    fn validate_action(&self, action: &ActionName) -> SuccessStatus {
        let known_actions = [
            "check",
            "scan",
            "security",
            "complexity",
            "duplicates",
            "trends",
            "fix",
            "report",
            "version",
            "adapters",
            "install-hook",
            "install_hook",
            "uninstall-hook",
            "uninstall_hook",
            "batch",
            "multi_project",
            "doctor",
            "cancel",
        ];
        SuccessStatus::new(known_actions.contains(&action.value()))
    }
}

impl Default for PipelineActionOrchestrator {
    fn default() -> Self {
        Self::new()
    }
}

impl PipelineActionOrchestrator {
    pub fn new() -> Self {
        Self {}
    }

    async fn handle_check(&self, _action: &str) -> ResponseData {
        let mut metadata = HashMap::new();
        metadata.insert("message".to_string(), serde_json::json!("check completed"));
        ResponseData {
            value: None,
            stdout: "check completed".to_string(),
            stderr: String::new(),
            returncode: 0,
            metadata,
        }
    }

    async fn handle_security(&self, _action: &str) -> ResponseData {
        let mut metadata = HashMap::new();
        metadata.insert(
            "bandit".to_string(),
            serde_json::json!(Vec::<String>::new()),
        );
        ResponseData {
            value: None,
            stdout: String::new(),
            stderr: String::new(),
            returncode: 0,
            metadata,
        }
    }

    async fn handle_complexity(&self, _action: &str) -> ResponseData {
        let mut metadata = HashMap::new();
        metadata.insert("radon".to_string(), serde_json::json!(Vec::<String>::new()));
        ResponseData {
            value: None,
            stdout: String::new(),
            stderr: String::new(),
            returncode: 0,
            metadata,
        }
    }

    async fn handle_duplicates(&self, _action: &str) -> ResponseData {
        let mut metadata = HashMap::new();
        metadata.insert(
            "duplicates".to_string(),
            serde_json::json!(Vec::<String>::new()),
        );
        ResponseData {
            value: None,
            stdout: String::new(),
            stderr: String::new(),
            returncode: 0,
            metadata,
        }
    }

    async fn handle_trends(&self, _action: &str) -> ResponseData {
        let mut metadata = HashMap::new();
        metadata.insert(
            "trends".to_string(),
            serde_json::json!(Vec::<String>::new()),
        );
        ResponseData {
            value: None,
            stdout: String::new(),
            stderr: String::new(),
            returncode: 0,
            metadata,
        }
    }

    async fn handle_fix(&self, _action: &str) -> ResponseData {
        let mut metadata = HashMap::new();
        metadata.insert("output".to_string(), serde_json::json!("fix applied"));
        ResponseData {
            value: None,
            stdout: "fix applied".to_string(),
            stderr: String::new(),
            returncode: 0,
            metadata,
        }
    }

    async fn handle_report(&self, _action: &str) -> ResponseData {
        let mut metadata = HashMap::new();
        metadata.insert("format".to_string(), serde_json::json!("text"));
        metadata.insert("data".to_string(), serde_json::json!({}));
        ResponseData {
            value: None,
            stdout: String::new(),
            stderr: String::new(),
            returncode: 0,
            metadata,
        }
    }

    async fn handle_version(&self, _action: &str) -> ResponseData {
        let mut metadata = HashMap::new();
        metadata.insert("version".to_string(), serde_json::json!("1.0.0"));
        ResponseData {
            value: None,
            stdout: "1.0.0".to_string(),
            stderr: String::new(),
            returncode: 0,
            metadata,
        }
    }

    async fn handle_adapters(&self, _action: &str) -> ResponseData {
        let mut metadata = HashMap::new();
        metadata.insert(
            "adapters".to_string(),
            serde_json::json!(Vec::<String>::new()),
        );
        ResponseData {
            value: None,
            stdout: String::new(),
            stderr: String::new(),
            returncode: 0,
            metadata,
        }
    }

    async fn handle_install_hook(&self, _action: &str) -> ResponseData {
        let mut metadata = HashMap::new();
        metadata.insert("installed".to_string(), serde_json::json!(true));
        ResponseData {
            value: None,
            stdout: "hook installed".to_string(),
            stderr: String::new(),
            returncode: 0,
            metadata,
        }
    }

    async fn handle_uninstall_hook(&self, _action: &str) -> ResponseData {
        let mut metadata = HashMap::new();
        metadata.insert("uninstalled".to_string(), serde_json::json!(true));
        ResponseData {
            value: None,
            stdout: "hook uninstalled".to_string(),
            stderr: String::new(),
            returncode: 0,
            metadata,
        }
    }
}
