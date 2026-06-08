use /* UNKNOWN: DoctorResultVO */ crate::project_setup::taxonomy_doctor_vo::DoctorResultVO;
use crate::source_parsing::taxonomy_path_vo::FilePath;
use crate::pipeline_jobs::taxonomy_action_vo::JobId;
use /* UNKNOWN: MaintenanceStatsVO */ crate::project_setup::taxonomy_stats_vo::MaintenanceStatsVO;
use async_trait::async_trait;

#[async_trait]
pub trait MaintenanceCommandsAggregate: Send + Sync {
    async fn stats(&self, project_path: &FilePath) -> MaintenanceStatsVO;
    async fn clean(&self);
    async fn update(&self);
    async fn doctor(&self) -> DoctorResultVO;
    async fn cancel(&self, job_id: JobId);
}
