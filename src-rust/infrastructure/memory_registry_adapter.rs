/// memory_registry_adapter — In-memory job tracking implementation.
use crate::contract::job_registry_port::IJobRegistryPort;
use crate::taxonomy::{Duration, ErrorMessage, JobId, ResponseData, SuccessStatus, JobError};
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

impl Default for MemoryJobRegistryAdapter {
    fn default() -> Self {
        Self::new()
    }
}

impl MemoryJobRegistryAdapter {
    pub fn new() -> Self {
        Self { jobs: Mutex::new(HashMap::new()) }
    }
}

#[async_trait::async_trait]
impl IJobRegistryPort for MemoryJobRegistryAdapter {
    async fn create_job(&self, action: &str) -> Result<JobId, JobError> {
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
        Ok(JobId::new(job_id))
    }

    async fn complete_job(&self, job_id: &JobId, result: &ResponseData) {
        let mut jobs = self.jobs.lock().await;
        if let Some(record) = jobs.get_mut(&job_id.value) {
            record.status = "completed".to_string();
            record.result = Some(format!("{:?}", result));
            record.completed_at = Some(chrono::Utc::now().to_rfc3339());
        }
    }

    async fn fail_job(&self, job_id: &JobId, error: &ErrorMessage) {
        let mut jobs = self.jobs.lock().await;
        if let Some(record) = jobs.get_mut(&job_id.value) {
            record.status = "failed".to_string();
            record.error = Some(error.value.clone());
            record.completed_at = Some(chrono::Utc::now().to_rfc3339());
        }
    }

    async fn list_jobs(&self) -> Vec<serde_json::Value> {
        let jobs = self.jobs.lock().await;
        jobs.values().map(|j| serde_json::json!({
            "id": j.id,
            "status": j.status,
            "action": j.action,
            "started_at": j.started_at,
            "completed_at": j.completed_at,
        })).collect()
    }

    async fn get_job(&self, job_id: &JobId) -> Option<JobId> {
        let jobs = self.jobs.lock().await;
        if jobs.contains_key(&job_id.value) {
            Some(job_id.clone())
        } else {
            None
        }
    }

    async fn cancel_job(&self, job_id: &JobId) -> SuccessStatus {
        let mut jobs = self.jobs.lock().await;
        if let Some(record) = jobs.get_mut(&job_id.value) {
            record.status = "cancelled".to_string();
            record.completed_at = Some(chrono::Utc::now().to_rfc3339());
            SuccessStatus::new(true)
        } else {
            SuccessStatus::new(false)
        }
    }

    async fn run_with_retry(
        &self,
        _operation: &str,
        _max_retries: u32,
        _base_delay: Duration,
    ) -> ResponseData {
        ResponseData {
            value: None,
            stdout: "success".to_string(),
            stderr: String::new(),
            returncode: 0,
            metadata: HashMap::new(),
        }
    }
}
