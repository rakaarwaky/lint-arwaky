// PURPOSE: Module declarations for auto-fix (fix processor, orchestrator, container)
pub use shared::auto_fix::taxonomy_fix_applied_event::FixApplied;
pub use shared::auto_fix::taxonomy_fix_vo::FixResult;
pub use shared::auto_fix::taxonomy_symbol_renamer_utility::SymbolRenamer;
pub use shared::auto_fix::contract_fix_protocol::IFixProtocol;
pub use shared::auto_fix::contract_fix_aggregate::LintFixOrchestratorAggregate;
pub mod capabilities_fix_processor;
pub use capabilities_fix_processor::LintFixProcessor;
pub mod agent_fix_orchestrator;
pub use agent_fix_orchestrator::FixOrchestrator;
pub mod root_auto_fix_container;
