// PURPOSE: Module declarations for naming-rules (checkers, orchestrator, container)
pub use shared::naming_rules::taxonomy_suffix_vo::{SuffixPolicyVO, SuffixVO};
pub use shared::naming_rules::contract_naming_runner_aggregate::INamingRunnerAggregate;
pub mod capabilities_naming_checker;
pub use capabilities_naming_checker::ArchNamingChecker;
pub mod agent_naming_orchestrator;
pub use agent_naming_orchestrator::NamingOrchestrator;
pub mod root_naming_container;
