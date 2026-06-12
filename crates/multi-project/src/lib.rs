// PURPOSE: Module declarations and re-exports for multi-project (orchestrator, aggregates, protocol, VOs)

// Re-export shared multi-project taxonomy types
pub use shared::multi_project::taxonomy_multi_project_vo::MultiProjectVO;
pub use shared::multi_project::taxonomy_summary_vo::{AggregatedResults, ProjectResult};

// Local modules
pub mod agent_project_orchestrator;
pub use agent_project_orchestrator::MultiProjectOrchestrator;
pub mod contract_orchestrator_aggregate;
pub use contract_orchestrator_aggregate::MultiProjectOrchestratorAggregate;
pub mod multi_project_container;