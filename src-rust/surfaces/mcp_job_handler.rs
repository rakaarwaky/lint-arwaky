use crate::taxonomy::job_action_vo::JobId;
use serde_json::json;
use std::sync::Arc;
use crate::contract::service_container_aggregate::ServiceContainerAggregate;

pub struct McpJobCommandsSurface;

impl Default for McpJobCommandsSurface {
    fn default() -> Self {
        Self::new()
    }
}

impl McpJobCommandsSurface {
    pub fn new() -> Self {
        Self
    }

    pub async fn check_status(
        &self,
        container: &Arc<dyn ServiceContainerAggregate>,
        job_id: Option<String>,
    ) -> Result<String, String> {
        let job_registry = container
            .get_job_registry()
            .ok_or_else(|| "Container not initialized".to_string())?;

        match job_id {
            None => {
                let jobs_list = job_registry.list_jobs().await;
                Ok(json!({ "jobs": jobs_list, "total": jobs_list.len() }).to_string())
            }
            Some(jid) => {
                let job_info = job_registry
                    .get_job(&JobId::new(&jid))
                    .await;
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
        container: &Arc<dyn ServiceContainerAggregate>,
        job_id: String,
    ) -> Result<String, String> {
        let job_registry = container
            .get_job_registry()
            .ok_or_else(|| "Container not initialized".to_string())?;

        let success = job_registry
            .cancel_job(&JobId::new(&job_id))
            .await;

        Ok(json!({
            "job_id": job_id,
            "status": if success.value { "cancelled" } else { "failed_to_cancel" },
            "success": success.value,
        })
        .to_string())
    }
}
