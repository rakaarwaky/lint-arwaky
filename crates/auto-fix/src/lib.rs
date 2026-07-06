// PURPOSE: Module declarations for auto-fix (fix processor, orchestrator, container)
pub mod capabilities_fix_processor;
pub use capabilities_fix_processor::LintFixProcessor;
pub mod agent_fix_orchestrator;
pub use agent_fix_orchestrator::FixOrchestrator;
pub mod root_auto_fix_container;
