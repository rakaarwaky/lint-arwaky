// PURPOSE: RegistryAggregate — aggregate trait for job registry operations (create, complete, fail, list)
use pipeline_jobs::contract_registry_port::IJobRegistryPort;
use pipeline_jobs::taxonomy_action_vo::ActionName;
use pipeline_jobs::taxonomy_action_vo::JobId;
use pipeline_jobs::taxonomy_job_vo::ResponseData;
use pipeline_jobs::taxonomy_job_vo::SuccessStatus;
use shared_common::taxonomy_common_error::ErrorMessage;
use shared_common::taxonomy_common_vo::Count;
/* UNKNOWN: ResponseDataList */
use shared_common::taxonomy_common_vo::ResponseDataList;
use shared_common::taxonomy_duration_vo::Duration;
use async_trait::async_trait;

#[async_trait]
pub trait JobRegistryAggregate: Send + Sync {
    fn port(&self) -> &dyn IJobRegistryPort;
    async fn create_job(&self, action: ActionName) -> JobId;
    async fn complete_job(&self, job_id: JobId, result: ResponseData);
    async fn fail_job(&self, job_id: JobId, error: ErrorMessage);
    async fn list_jobs(&self) -> ResponseDataList;
    async fn get_job(&self, job_id: JobId) -> Option<ResponseData>;
    async fn cancel_job(&self, job_id: JobId) -> SuccessStatus;
    async fn run_with_retry(
        &self,
        operation: &ActionName,
        max_retries: Count,
        base_delay: Duration,
    ) -> ResponseData;
}
