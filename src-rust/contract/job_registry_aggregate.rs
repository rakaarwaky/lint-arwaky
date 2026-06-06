use crate::contract::IJobRegistryPort;
use crate::taxonomy::ActionName;
use crate::taxonomy::Count;
use crate::taxonomy::Duration;
use crate::taxonomy::ErrorMessage;
use crate::taxonomy::JobId;
use crate::taxonomy::ResponseData;
use crate::taxonomy::ResponseDataList;
use crate::taxonomy::SuccessStatus;
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
