use crate::contract::job_registry_aggregate::JobRegistryAggregate;
use crate::taxonomy::job_action_vo::JobId;
use serde_json::json;
use std::sync::Arc;

pub struct McpJobCommandsSurface;

impl McpJobCommandsSurface {
    pub fn new() -> Self {
        Self
    }

    pub async fn check_status(
        &self,
        container: &Arc<crate::contract::service_container_aggregate::ServiceContainerAggregate>,
        job_id: Option<String>,
    ) -> Result<String, String> {
        let job_registry = container
            .get::<Arc<dyn JobRegistryAggregate>>()
            .ok_or_else(|| "Container not initialized".to_string())?;

        match job_id {
            None => {
                let all_jobs_vo = job_registry.list_jobs().await.map_err(|e| e.to_string())?;
                let all_jobs = all_jobs_vo;
                let jobs_list: Vec<serde_json::Value> = all_jobs
                    .iter()
                    .map(|(jid, info)| {
                        json!({
                            "job_id": jid,
                            "status": info.status,
                            "action": info.action,
                        })
                    })
                    .collect();
                Ok(json!({ "jobs": jobs_list, "total": jobs_list.len() }).to_string())
            }
            Some(jid) => {
                let job_info = job_registry
                    .get_job(&JobId::new(&jid))
                    .await
                    .map_err(|e| e.to_string())?;
                match job_info {
                    Some(info) => Ok(json!({
                        "job_id": jid,
                        "status": info.status,
                        "action": info.action,
                        "started_at": info.started_at,
                        "completed_at": info.completed_at,
                        "result": info.result,
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
        container: &Arc<crate::contract::service_container_aggregate::ServiceContainerAggregate>,
        job_id: String,
    ) -> Result<String, String> {
        let job_registry = container
            .get::<Arc<dyn JobRegistryAggregate>>()
            .ok_or_else(|| "Container not initialized".to_string())?;

        let success = job_registry
            .cancel_job(&JobId::new(&job_id))
            .await
            .map_err(|e| e.to_string())?;

        Ok(json!({
            "job_id": job_id,
            "status": if success { "cancelled" } else { "failed_to_cancel" },
            "success": success,
        })
        .to_string())
    }
}
