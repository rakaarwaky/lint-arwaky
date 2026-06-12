// PURPOSE: Module declarations for pipeline-jobs (orchestrators, adapters, container)
pub mod agent_job_container;
pub use agent_job_container::PipelineJobRegistry;
pub mod agent_pipeline_orchestrator;
pub use agent_pipeline_orchestrator::PipelineActionOrchestrator;
pub mod agent_pipeline_extended_orchestrator;
pub use agent_pipeline_extended_orchestrator::PipelineExtendedOrchestrator;
pub mod infrastructure_registry_adapter;
pub use infrastructure_registry_adapter::MemoryJobRegistryAdapter;
pub mod root_pipeline_container;
