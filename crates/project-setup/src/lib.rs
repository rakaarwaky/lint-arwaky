// PURPOSE: Module declarations for project-setup (orchestrator, processor, container)
pub mod agent_setup_orchestrator;
pub use agent_setup_orchestrator::SetupManagementOrchestrator;
pub mod capabilities_setup_processor;
pub use capabilities_setup_processor::SetupManagementProcessor;
pub mod capabilities_setup_installer_adapter;
pub mod root_project_setup_container;
pub use capabilities_setup_installer_adapter::SetupInstallerAdapter;
