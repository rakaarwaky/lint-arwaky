use crate::taxonomy::source_path_vo::FilePath;
use super::job_registry_port::IJobRegistryPort;

pub trait WatchExecutionOrchestratorAggregate: Send + Sync {
    fn root_path(&self) -> Option<&FilePath>;
    fn job_registry(&self) -> &dyn IJobRegistryPort;
}
