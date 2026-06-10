// PURPOSE: Module: multi-project module declarations and re-exports
pub mod agent_project_orchestrator;
pub use agent_project_orchestrator::MultiProjectOrchestrator;
pub mod contract_rule_engine_protocol;
pub use contract_rule_engine_protocol::{
    IArchRuleEngineProtocol, IConfigRulesProtocol, IMetricAnalyzerProtocol,
};
pub mod contract_project_aggregate;
pub use contract_project_aggregate::MultiProjectAggregate;
pub mod contract_orchestrator_aggregate;
pub use contract_orchestrator_aggregate::MultiProjectOrchestratorAggregate;
pub mod taxonomy_summary_vo;
pub use taxonomy_summary_vo::{AggregatedResults, ProjectResult};
