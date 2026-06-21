// PURPOSE: Module declarations for naming-rules (checkers, orchestrator, container)
pub mod capabilities_naming_convention_checker;
pub mod capabilities_suffix_prefix_checker;
pub mod infrastructure_filesystem_adapter;
pub use infrastructure_filesystem_adapter::OSFileSystemAdapter;
pub mod agent_naming_orchestrator;
pub use agent_naming_orchestrator::NamingOrchestrator;
pub mod root_naming_rules_container;
