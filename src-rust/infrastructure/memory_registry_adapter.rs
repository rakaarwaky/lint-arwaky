/// memory_registry_adapter — In-memory job tracking implementation.
use crate::contract::IJobRegistryPort;
use crate::taxonomy::{Count, Duration, ErrorMessage, Identity, JobId, MetadataVO, ResponseData, SuccessStatus, BooleanVO, JobError, JobStatus};
use std::collections::HashMap;
use tokio::sync::Mutex;

#[derive(Debug, Clone)]
struct JobRecord {
    id: String,
    status: String,
    action: String,
    started_at: String,
    result: Option<String>,
    error: Option<String>,
    completed_at: Option<String>,
}

pub struct MemoryJobRegistryAdapter {
    jobs: Mutex<HashMap<String, JobRecord>>,
}

impl MemoryJobRegistryAdapter {
    pub fn new() -> Self {
        Self { jobs: Mutex::new(HashMap::new()) }
    }

    pub async fn create_job(&self, action: &str) -> JobId {
        let job_id = format!("{:x}", rand::random::<u32>());
        let record = JobRecord {
            id: job_id.clone(),
            status: "running".to_string(),
            action: action.to_string(),
            started_at: chrono::Utc::now().to_rfc3339(),
            result: None,
            error: None,
            completed_at: None,
        };
        self.jobs.lock().await.insert(job_id.clone(), record);
        JobId::new(job_id)
    }

    pub async fn complete_job(&self, job_id: &JobId, result: &ResponseData) {
        let mut jobs = self.jobs.lock().await;
        if let Some(record) = jobs.get_mut(&job_id.value) {
            record.status = "completed".to_string();
            record.result = Some(result.value.to_string());
            record.completed_at = Some(chrono::Utc::now().to_rfc3339());
        }
    }

    pub async fn fail_job(&self, job_id: &JobId, error: &ErrorMessage) {
        let mut jobs = self.jobs.lock().await;
        if let Some(record) = jobs.get_mut(&job_id.value) {
            record.status = "failed".to_string();
            record.error = Some(error.value.clone());
            record.completed_at = Some(chrono::Utc::now().to_rfc3339());
        }
    }

    pub async fn list_jobs(&self) -> Vec<serde_json::Value> {
        let jobs = self.jobs.lock().await;
        jobs.values().map(|j| serde_json::json!({
            "id": j.id,
            "status": j.status,
            "action": j.action,
            "started_at": j.started_at,
            "completed_at": j.completed_at,
        })).collect()
    }

    pub async fn cancel_job(&self, job_id: &JobId) -> SuccessStatus {
        let mut jobs = self.jobs.lock().await;
        if let Some(record) = jobs.get_mut(&job_id.value) {
            record.status = "cancelled".to_string();
            record.completed_at = Some(chrono::Utc::now().to_rfc3339());
            SuccessStatus::new(BooleanVO::new(true))
        } else {
            SuccessStatus::new(BooleanVO::new(false))
        }
    }
}

#[async_trait::async_trait]
impl IJobRegistryPort for MemoryJobRegistryAdapter {
    async fn create_job(&self, action: &str) -> Result<JobId, JobError> {
        unimplemented!()
    }
    async fn complete_job(&self, job_id: &JobId, result: &ResponseData) {
        unimplemented!()
    }
    async fn fail_job(&self, job_id: &JobId, error: &ErrorMessage) {
        unimplemented!()
    }
    async fn list_jobs(&self) -> Vec<serde_json::Value> {
        unimplemented!()
    }
    async fn get_job(&self, job_id: &JobId) -> Option<JobId> {
        unimplemented!()
    }
    async fn cancel_job(&self, job_id: &JobId) -> SuccessStatus {
        unimplemented!()
    }
    async fn run_with_retry(
        &self,
        operation: &str,
        max_retries: u32,
        base_delay: Duration,
    ) -> ResponseData {
        unimplemented!()
    }
}

