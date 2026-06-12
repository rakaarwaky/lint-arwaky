// PURPOSE: Module declarations and re-exports for project-setup (orchestrator, processor, aggregates, VOs)
pub mod agent_setup_orchestrator;
pub use agent_setup_orchestrator::SetupManagementOrchestrator;
pub mod capabilities_setup_processor;
pub use capabilities_setup_processor::SetupManagementProcessor;
pub mod contract_setup_aggregate;
pub use contract_setup_aggregate::SetupManagementAggregate;
pub mod contract_setup_protocol;
pub use contract_setup_protocol::ISetupManagementProtocol;
pub mod taxonomy_doctor_vo;
pub use taxonomy_doctor_vo::DoctorResultVO;
pub mod taxonomy_language_vo;
pub use taxonomy_language_vo::{LanguageSource, ProjectLanguage};
pub mod taxonomy_stats_vo;
pub use taxonomy_stats_vo::MaintenanceStatsVO;
pub mod root_setup_container;
pub mod agent_maintenance_orchestrator;
pub use agent_maintenance_orchestrator::MaintenanceCommandsOrchestrator;
