// PURPOSE: Module declarations and re-exports for multi-project (orchestrator, aggregates, protocol, VOs)
pub mod agent_project_orchestrator;
pub use agent_project_orchestrator::MultiProjectOrchestrator;
pub mod taxonomy_multi_project_vo;
pub use taxonomy_multi_project_vo::MultiProjectVO;
pub mod contract_orchestrator_aggregate;
pub use contract_orchestrator_aggregate::MultiProjectOrchestratorAggregate;
pub mod taxonomy_summary_vo;
pub use taxonomy_summary_vo::{AggregatedResults, ProjectResult};
