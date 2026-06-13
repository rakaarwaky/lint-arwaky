// PURPOSE: JobController — MCP surface for job lifecycle management endpoints
use serde_json::json;
use shared::pipeline_jobs::contract_registry_port::IJobRegistryPort;
use shared::pipeline_jobs::taxonomy_action_vo::JobId;
use std::sync::Arc;

pub struct McpJobCommandsSurface {}

impl Default for McpJobCommandsSurface {
    fn default() -> Self {
        Self::new()
    }
}

impl McpJobCommandsSurface {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn check_status(
        &self,
        job_registry: &Arc<dyn IJobRegistryPort>,
        job_id: Option<String>,
    ) -> Result<String, String> {
        match job_id {
            None => {
                let jobs_list = job_registry.list_jobs().await;
                Ok(json!({ "jobs": jobs_list, "total": jobs_list.len() }).to_string())
            }
            Some(jid) => {
                let job_info = job_registry.get_job(&JobId::new(&jid)).await;
                match job_info {
                    Some(_info) => Ok(json!({
                        "job_id": jid,
                        "status": "running",
                        "action": "check",
                    })
                    .to_string()),
                    None => Ok(json!({
                        "error": format!("Job '{}' not found", jid),
                        "status": "not_found",
                    })
                    .to_string()),
                }
            }
        }
    }

    pub async fn cancel_job(
        &self,
        job_registry: &Arc<dyn IJobRegistryPort>,
        job_id: String,
    ) -> Result<String, String> {
        let success = job_registry.cancel_job(&JobId::new(&job_id)).await;

        Ok(json!({
            "job_id": job_id,
            "status": if success.value { "cancelled" } else { "failed_to_cancel" },
            "success": success.value,
        })
        .to_string())
    }
}
