// PURPOSE: WatchExecutionOrchestratorAggregate — aggregate trait for watch execution orchestration
use crate::pipeline_jobs::contract_registry_port::IJobRegistryPort;
use crate::source_parsing::taxonomy_path_vo::FilePath;

pub trait WatchExecutionOrchestratorAggregate: Send + Sync {
    fn root_path(&self) -> Option<&FilePath>;
    fn job_registry(&self) -> &dyn IJobRegistryPort;
}
