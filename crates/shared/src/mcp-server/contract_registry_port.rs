// PURPOSE: IJobRegistryPort — port trait for job lifecycle management (create, complete, list, cancel)

use crate::common::taxonomy_common_error::ErrorMessage;
use crate::common::taxonomy_common_vo::Count;
use crate::common::taxonomy_common_vo::ResponseDataList;
use crate::common::taxonomy_duration_vo::Duration;
use crate::mcp_server::taxonomy_action_vo::ActionName;
use crate::mcp_server::taxonomy_action_vo::JobId;
use crate::mcp_server::taxonomy_job_vo::ResponseData;
use crate::mcp_server::taxonomy_job_vo::SuccessStatus;
use crate::mcp_server::taxonomy_registry_error::JobError;
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
