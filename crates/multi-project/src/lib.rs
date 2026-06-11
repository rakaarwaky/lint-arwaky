// PURPOSE: Module declarations and re-exports for multi-project (orchestrator, aggregates, protocol, VOs)
pub mod contract_orchestrator_aggregate;
pub mod agent_project_orchestrator;
pub mod multi_project_container;

pub use agent_project_orchestrator::MultiProjectOrchestrator;
pub use contract_orchestrator_aggregate::MultiProjectOrchestratorAggregate;
pub use shared::taxonomy_multi_project_vo::MultiProjectVO;
pub use shared::taxonomy_summary_vo::{AggregatedResults, ProjectResult};
