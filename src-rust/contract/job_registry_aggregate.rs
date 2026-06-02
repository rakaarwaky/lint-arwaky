use async_trait::async_trait;
use super::*;

#[async_trait]
pub trait JobRegistryAggregate: Send + Sync {
    fn port(&self) -> &dyn IJobRegistryPort;
    async fn create_job(&self, action: ActionName) -> JobId;
    async fn complete_job(&self, job_id: JobId, result: serde_json::Value);
    async fn fail_job(&self, job_id: JobId, error: ErrorMessage);
    async fn list_jobs(&self) -> serde_json::Value;
    async fn get_job(&self, job_id: JobId) -> Option<serde_json::Value>;
    async fn cancel_job(&self, job_id: JobId) -> SuccessStatus;
    async fn run_with_retry(&self, operation: &str, max_retries: Count, base_delay: f64) -> serde_json::Value;
}
