// PURPOSE: Module declarations and re-exports for auto-fix (protocol, aggregate, processor, orchestrator)
pub mod contract_fix_protocol;
pub use contract_fix_protocol::IFixProtocol;
pub mod contract_fix_aggregate;
pub use contract_fix_aggregate::LintFixOrchestratorAggregate;
pub mod capabilities_fix_processor;
pub use capabilities_fix_processor::LintFixProcessor;
pub mod agent_fix_orchestrator;
pub use agent_fix_orchestrator::FixOrchestrator;
