// agent_job_registry — Implementation of JobRegistryAggregate (Agent Layer).
use crate::contract::{JobRegistryAggregate, IJobRegistryPort};
use crate::taxonomy::{JobId, ActionName, Identity, MetadataVO, SuccessStatus, ErrorMessage, ResponseData, Count, Duration};

use async_trait::async_trait;
use std::collections::HashMap;

pub struct JobRegistry {
    port: Box<dyn IJobRegistryPort + Send + Sync>,
}

#[async_trait]
impl JobRegistryAggregate for JobRegistry {
    fn port(&self) -> &dyn IJobRegistryPort {
        &*self.port
    }

    async fn create_job(&self, action: ActionName) -> JobId {
        self.create_job_old(action).await.unwrap_or_else(|_| JobId::new("error"))
    }

    async fn complete_job(&self, job_id: JobId, _result: serde_json::Value) {
        let response_data = ResponseData {
            value: None,
            stdout: String::new(),
            stderr: String::new(),
            returncode: 0,
            metadata: HashMap::new(),
        };
        self.complete_job_old(job_id, response_data).await
    }

    async fn fail_job(&self, job_id: JobId, error: ErrorMessage) {
        self.fail_job_old(job_id, error).await
    }

    async fn list_jobs(&self) -> serde_json::Value {
        let jobs = self.list_jobs_old().await;
        serde_json::Value::Array(jobs)
    }

    async fn get_job(&self, job_id: JobId) -> Option<serde_json::Value> {
        let job = self.get_job_old(job_id).await;
        job.map(|j| serde_json::json!({ "id": j.value }))
    }

    async fn cancel_job(&self, job_id: JobId) -> SuccessStatus {
        self.cancel_job_old(job_id).await
    }

    async fn run_with_retry(&self, operation: &str, max_retries: Count, base_delay: f64) -> serde_json::Value {
        let res = self.run_with_retry_old(
            operation.to_string(),
            max_retries.value as u32,
            Duration::new(base_delay),
        ).await;
        serde_json::to_value(res).unwrap_or(serde_json::Value::Null)
    }
}

impl JobRegistry {
    pub fn new(port: Box<dyn IJobRegistryPort + Send + Sync>) -> Self {
        Self { port }
    }

    pub async fn create_job_old(&self, action: ActionName) -> Result<JobId, crate::taxonomy::JobError> {
        self.port.create_job(&action.value).await
    }

    pub async fn complete_job_old(&self, job_id: JobId, result: ResponseData) {
        self.port.complete_job(&job_id, &result).await
    }

    pub async fn fail_job_old(&self, job_id: JobId, error: ErrorMessage) {
        self.port.fail_job(&job_id, &error).await
    }

    pub async fn list_jobs_old(&self) -> Vec<serde_json::Value> {
        self.port.list_jobs().await
    }

    pub async fn get_job_old(&self, job_id: JobId) -> Option<JobId> {
        self.port.get_job(&job_id).await
    }

    pub async fn cancel_job_old(&self, job_id: JobId) -> SuccessStatus {
        self.port.cancel_job(&job_id).await
    }

    pub async fn run_with_retry_old(
        &self,
        operation: String,
        max_retries: u32,
        base_delay: Duration,
    ) -> ResponseData {
        self.port.run_with_retry(&operation, max_retries, base_delay).await
    }
}
