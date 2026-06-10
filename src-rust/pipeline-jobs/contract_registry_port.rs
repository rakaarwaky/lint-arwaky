// PURPOSE: Port: Interface for Registry

use crate::pipeline_jobs::taxonomy_action_vo::ActionName;
use crate::pipeline_jobs::taxonomy_action_vo::JobId;
use crate::pipeline_jobs::taxonomy_job_vo::ResponseData;
use crate::pipeline_jobs::taxonomy_job_vo::SuccessStatus;
use crate::pipeline_jobs::taxonomy_registry_error::JobError;
use crate::shared_common::taxonomy_common_error::ErrorMessage;
use crate::shared_common::taxonomy_common_vo::Count;
use crate::shared_common::taxonomy_common_vo::ResponseDataList;
use crate::shared_common::taxonomy_duration_vo::Duration;
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
