use super::*;

pub trait PipelineExecutionOrchestratorAggregate: Send + Sync {
    fn job_registry(&self) -> &dyn IJobRegistryPort;
}
