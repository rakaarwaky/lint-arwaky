//! Job registry aggregation for pipeline jobs.
use crate::contract::{IJobRegistryPort, JobRegistryAggregate};
use crate::infrastructure::MemoryJobRegistryAdapter;
use crate::taxonomy::{
    ActionName, Count, Duration, ErrorMessage, JobId, ResponseData, ResponseDataList, SuccessStatus,
};
use async_trait::async_trait;

pub struct PipelineJobRegistry {
    adapter: MemoryJobRegistryAdapter,
}

impl PipelineJobRegistry {
    pub fn new() -> Self {
        Self {
            adapter: MemoryJobRegistryAdapter::new(),
        }
    }
}

#[async_trait]
impl JobRegistryAggregate for PipelineJobRegistry {
    fn port(&self) -> &dyn IJobRegistryPort {
        &self.adapter
    }

    async fn create_job(&self, _action: ActionName) -> JobId {
        JobId::new("stub")
    }

    async fn complete_job(&self, job_id: JobId, result: ResponseData) {
        self.adapter.complete_job(&job_id, &result).await;
    }

    async fn fail_job(&self, job_id: JobId, error: ErrorMessage) {
        self.adapter.fail_job(&job_id, &error).await;
    }

    async fn list_jobs(&self) -> ResponseDataList {
        ResponseDataList { values: vec![] }
    }

    async fn get_job(&self, _job_id: JobId) -> Option<ResponseData> {
        None
    }

    async fn cancel_job(&self, _job_id: JobId) -> SuccessStatus {
        SuccessStatus::new(false)
    }

    async fn run_with_retry(
        &self,
        _operation: &ActionName,
        _max_retries: Count,
        _base_delay: Duration,
    ) -> ResponseData {
        ResponseData {
            value: None,
            stdout: "success".to_string(),
            stderr: String::new(),
            returncode: 0,
            metadata: std::collections::HashMap::new(),
        }
    }
}
