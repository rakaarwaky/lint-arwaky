// PURPOSE: Module declarations and re-exports for pipeline-jobs (orchestrators, aggregates, adapters, VOs, errors)
pub mod agent_job_container;
pub use agent_job_container::PipelineJobRegistry;
pub mod agent_pipeline_orchestrator;
pub use agent_pipeline_orchestrator::PipelineActionOrchestrator;

pub mod agent_pipeline_extended_orchestrator;
pub use agent_pipeline_extended_orchestrator::PipelineExtendedOrchestrator;
pub mod contract_dispatcher_aggregate;
pub use contract_dispatcher_aggregate::PipelineActionDispatcherAggregate;
pub mod contract_extended_aggregate;
pub use contract_extended_aggregate::PipelineExtendedOrchestratorAggregate;
pub mod contract_output_aggregate;
pub use contract_output_aggregate::PipelineOutputAggregate;
pub mod contract_registry_aggregate;
pub use contract_registry_aggregate::JobRegistryAggregate;
pub mod contract_registry_port;
pub use contract_registry_port::IJobRegistryPort;
pub mod infrastructure_registry_adapter;
pub use infrastructure_registry_adapter::MemoryJobRegistryAdapter;
pub mod taxonomy_action_vo;
pub use taxonomy_action_vo::{ActionArgs, ActionName, JobId};
pub mod taxonomy_job_vo;
pub use taxonomy_job_vo::{
    AdapterMetadata, EnvContentVO, JobStatus, LintStatusActionArgs, McpConfigVO, ResponseData,
    SuccessStatus,
};
pub mod taxonomy_registry_error;
pub use taxonomy_registry_error::JobError;
