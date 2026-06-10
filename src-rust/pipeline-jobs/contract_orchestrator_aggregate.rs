// PURPOSE: PipelineOrchestratorAggregate — aggregate trait for main pipeline orchestration
use crate::pipeline_jobs::contract_registry_port::IJobRegistryPort;

pub trait PipelineExecutionOrchestratorAggregate: Send + Sync {
    fn job_registry(&self) -> &dyn IJobRegistryPort;
}
