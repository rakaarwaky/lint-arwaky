//! Watch orchestrators — CLI watch command + file-change execution.

use std::collections::HashMap;
use std::sync::OnceLock;

use async_trait::async_trait;

use crate::file_watch::contract_commands_aggregate::WatchCommandsAggregate;
use crate::file_watch::contract_orchestrator_aggregate::WatchExecutionOrchestratorAggregate;
use crate::file_watch::contract_watch_aggregate::DirectoryWatchAggregate;
use crate::file_watch::taxonomy_result_vo::WatchResult;
use crate::pipeline_jobs::contract_registry_port::IJobRegistryPort;
use crate::pipeline_jobs::taxonomy_action_vo::ActionName;
use crate::pipeline_jobs::taxonomy_action_vo::JobId;
use crate::pipeline_jobs::taxonomy_job_vo::ResponseData;
use crate::pipeline_jobs::taxonomy_job_vo::SuccessStatus;
use crate::pipeline_jobs::taxonomy_registry_error::JobError;
use crate::shared_common::taxonomy_common_error::ErrorMessage;
use crate::shared_common::taxonomy_common_vo::Count;
use crate::shared_common::taxonomy_common_vo::ResponseDataList;
use crate::shared_common::taxonomy_common_vo::Score;
use crate::shared_common::taxonomy_duration_vo::Duration;
use crate::shared_common::taxonomy_message_vo::ComplianceStatus;
use crate::source_parsing::taxonomy_path_vo::FilePath;

struct SimpleJobRegistry {}
#[async_trait]
impl IJobRegistryPort for SimpleJobRegistry {
    async fn create_job(&self, _action: ActionName) -> Result<JobId, JobError> {
        Ok(JobId::new("stub"))
    }
    async fn complete_job(&self, _job_id: &JobId, _result: &ResponseData) {}
    async fn fail_job(&self, _job_id: &JobId, _error: &ErrorMessage) {}
    async fn list_jobs(&self) -> ResponseDataList {
        ResponseDataList { values: vec![] }
    }
    async fn get_job(&self, _job_id: &JobId) -> Option<JobId> {
        None
    }
    async fn cancel_job(&self, _job_id: &JobId) -> SuccessStatus {
        SuccessStatus::new(true)
    }
    async fn run_with_retry(
        &self,
        _operation: ActionName,
        _max_retries: Count,
        _base_delay: Duration,
    ) -> ResponseData {
        ResponseData::default()
    }
}

static WATCH_JOB_REGISTRY: OnceLock<SimpleJobRegistry> = OnceLock::new();

pub struct WatchCommandsOrchestrator {
    execution: WatchExecutionOrchestrator,
}

#[async_trait]
impl WatchCommandsAggregate for WatchCommandsOrchestrator {
    fn root_path(&self) -> Option<&FilePath> {
        None
    }

    async fn watch(&self, path: &FilePath) {
        self.execution.process_event(path);
    }
}

impl Default for WatchCommandsOrchestrator {
    fn default() -> Self {
        Self::new()
    }
}

impl WatchCommandsOrchestrator {
    pub fn new() -> Self {
        Self {
            execution: WatchExecutionOrchestrator::new(),
        }
    }
}

pub struct WatchExecutionOrchestrator {}

impl WatchExecutionOrchestratorAggregate for WatchExecutionOrchestrator {
    fn root_path(&self) -> Option<&FilePath> {
        None
    }

    fn job_registry(&self) -> &dyn IJobRegistryPort {
        WATCH_JOB_REGISTRY.get_or_init(|| SimpleJobRegistry {})
    }
}

impl Default for WatchExecutionOrchestrator {
    fn default() -> Self {
        Self::new()
    }
}

impl WatchExecutionOrchestrator {
    pub fn new() -> Self {
        Self {}
    }

    pub fn is_available(&self) -> bool {
        true
    }

    pub async fn execute(&self, _request: &DirectoryWatchAggregate) -> WatchResult {
        WatchResult {
            file: FilePath::new(".".to_string()).unwrap_or_default(),
            score: Score::new(100.0),
            is_passing: ComplianceStatus::new(true),
        }
    }

    pub fn process_event(&self, file_path: &FilePath) -> HashMap<String, serde_json::Value> {
        let mut result = HashMap::new();
        result.insert("file".to_string(), serde_json::json!(file_path.value));
        result.insert("score".to_string(), serde_json::json!(0.0));
        result.insert("is_passing".to_string(), serde_json::json!(false));
        result
    }
}
