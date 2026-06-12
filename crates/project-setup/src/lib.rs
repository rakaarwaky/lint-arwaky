// PURPOSE: Module declarations for project-setup (orchestrator, processor, container)
pub mod agent_setup_orchestrator;
pub use agent_setup_orchestrator::SetupManagementOrchestrator;
pub mod capabilities_setup_processor;
pub use capabilities_setup_processor::SetupManagementProcessor;
pub mod agent_maintenance_orchestrator;
pub use agent_maintenance_orchestrator::MaintenanceCommandsOrchestrator;
pub mod root_setup_container;
