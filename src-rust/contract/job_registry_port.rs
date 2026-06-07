// job_registry_port — Port for job tracking and lifecycle management.
use crate::taxonomy::{
    ActionName, Count, Duration, ErrorMessage, JobError, JobId, ResponseData, ResponseDataList,
    SuccessStatus,
};
use async_trait::async_trait;

#[async_trait]
pub trait IJobRegistryPort: Send + Sync {
    /// Register a new job and return its ID.
    async fn create_job(&self, action: ActionName) -> Result<JobId, JobError>;

    /// Mark job as completed.
    async fn complete_job(&self, job_id: &JobId, result: &ResponseData);

    /// Mark job as failed.
    async fn fail_job(&self, job_id: &JobId, error: &ErrorMessage);

    /// Return all jobs.
    async fn list_jobs(&self) -> ResponseDataList;

    /// Return a single job or None.
    async fn get_job(&self, job_id: &JobId) -> Option<JobId>;

    /// Cancel a running job. Returns SuccessStatus if cancelled.
    async fn cancel_job(&self, job_id: &JobId) -> SuccessStatus;

    /// Execute async function with exponential backoff retry.
    async fn run_with_retry(
        &self,
        operation: ActionName,
        max_retries: Count,
        base_delay: Duration,
    ) -> ResponseData;
}
