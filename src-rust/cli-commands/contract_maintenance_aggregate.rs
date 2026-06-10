// PURPOSE: Aggregate: MaintenanceCommandsAggregate trait — contract for maintenance operations (stats, doctor, clean, update, cancel)
use crate::pipeline_jobs::taxonomy_action_vo::JobId;
use crate::project_setup::taxonomy_doctor_vo::DoctorResultVO;
use crate::project_setup::taxonomy_stats_vo::MaintenanceStatsVO;
use crate::cli_commands::contract_executor_port::ICommandExecutorPort as _;
use crate::source_parsing::taxonomy_path_vo::FilePath;
use async_trait::async_trait;

#[async_trait]
pub trait MaintenanceCommandsAggregate: Send + Sync {
    async fn stats(&self, project_path: &FilePath) -> MaintenanceStatsVO;
    async fn clean(&self);
    async fn update(&self);
    async fn doctor(&self) -> DoctorResultVO;
    async fn cancel(&self, job_id: JobId);
}
