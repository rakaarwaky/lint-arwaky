pub use shared::auto_fix::IFixProtocol;
pub use shared::auto_fix::LintFixOrchestratorAggregate;

pub mod agent_fix_orchestrator;
pub use agent_fix_orchestrator::FixOrchestrator;

pub mod capabilities_file_adapter;
pub use capabilities_file_adapter::FileAdapter;

pub mod capabilities_fix_processor;
pub use capabilities_fix_processor::LintFixProcessor;

pub mod root_auto_fix_container;
