use crate::contract::IJobRegistryPort;

pub trait PipelineExecutionOrchestratorAggregate: Send + Sync {
    fn job_registry(&self) -> &dyn IJobRegistryPort;
}
