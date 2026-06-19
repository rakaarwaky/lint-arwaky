use shared::common::taxonomy_common_error::ErrorMessage;
use shared::common::taxonomy_common_vo::{Count, ResponseDataList};
use shared::common::taxonomy_duration_vo::Duration;
use shared::mcp_server::contract_registry_port::IJobRegistryPort;
use shared::mcp_server::taxonomy_action_vo::{ActionName, JobId};
use shared::mcp_server::taxonomy_job_vo::{ResponseData, SuccessStatus};
use shared::mcp_server::taxonomy_registry_error::JobError;
use std::collections::HashMap;

pub struct InMemoryJobRegistry {
    _jobs: HashMap<String, JobState>,
}

struct JobState {
    _id: JobId,
    _action: ActionName,
    _result: Option<String>,
}

impl Default for InMemoryJobRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl InMemoryJobRegistry {
    pub fn new() -> Self {
        Self {
            _jobs: HashMap::new(),
        }
    }
}

#[async_trait::async_trait]
impl IJobRegistryPort for InMemoryJobRegistry {
    async fn create_job(&self, _action: ActionName) -> Result<JobId, JobError> {
        Ok(JobId::new("mock-job"))
    }

    async fn complete_job(&self, _job_id: &JobId, _result: &ResponseData) {}

    async fn fail_job(&self, _job_id: &JobId, _error: &ErrorMessage) {}

    async fn list_jobs(&self) -> ResponseDataList {
        ResponseDataList::new(Vec::new())
    }

    async fn get_job(&self, _job_id: &JobId) -> Option<JobId> {
        None
    }

    async fn cancel_job(&self, _job_id: &JobId) -> SuccessStatus {
        SuccessStatus::new(false)
    }

    async fn run_with_retry(
        &self,
        _operation: ActionName,
        _max_retries: Count,
        _base_delay: Duration,
    ) -> ResponseData {
        ResponseData::new()
    }
}
