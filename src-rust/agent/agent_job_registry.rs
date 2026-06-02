// agent_job_registry — Implementation of JobRegistryAggregate (Agent Layer).
use crate::contract::{JobRegistryAggregate, IJobRegistryPort};
use crate::taxonomy::{JobId, ActionName, Identity, MetadataVO, SuccessStatus, ErrorMessage, ResponseData, Count, Duration};

pub struct JobRegistry {
    port: Box<dyn IJobRegistryPort + Send + Sync>,
}

impl JobRegistry {
    pub fn new(port: Box<dyn IJobRegistryPort + Send + Sync>) -> Self {
        Self { port }
    }

    pub async fn create_job(&self, action: ActionName) -> JobId {
        self.port.create_job(action).await
    }

    pub async fn complete_job(&self, job_id: JobId, result: String) {
        self.port.complete_job(job_id, result).await
    }

    pub async fn fail_job(&self, job_id: JobId, error: String) {
        self.port.fail_job(job_id, error).await
    }

    pub async fn list_jobs(&self) -> MetadataVO {
        self.port.list_jobs().await
    }

    pub async fn get_job(&self, job_id: JobId) -> Option<String> {
        self.port.get_job(job_id).await
    }

    pub async fn cancel_job(&self, job_id: JobId) -> SuccessStatus {
        self.port.cancel_job(job_id).await
    }

    pub async fn run_with_retry(
        &self,
        operation: String,
        max_retries: Count,
        base_delay: Duration,
    ) -> ResponseData {
        self.port.run_with_retry(operation, max_retries, base_delay).await
    }
}

impl JobRegistryAggregate for JobRegistry {}
