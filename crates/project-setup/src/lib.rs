// PURPOSE: Module declarations for project-setup (orchestrator, processor, container)
pub use shared::project_setup::contract_setup_aggregate::SetupManagementAggregate;
pub use shared::project_setup::contract_setup_protocol::ISetupManagementProtocol;
pub use shared::project_setup::taxonomy_doctor_vo::DoctorResultVO;
pub use shared::project_setup::taxonomy_language_vo::{LanguageSource, ProjectLanguage};
pub use shared::project_setup::taxonomy_stats_vo::MaintenanceStatsVO;
pub mod agent_setup_orchestrator;
pub use agent_setup_orchestrator::SetupManagementOrchestrator;
pub mod capabilities_setup_processor;
pub use capabilities_setup_processor::SetupManagementProcessor;
pub mod agent_maintenance_orchestrator;
pub use agent_maintenance_orchestrator::MaintenanceCommandsOrchestrator;
pub mod root_setup_container;
