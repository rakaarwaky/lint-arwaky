pub use shared::maintenance::IAdapterHealthProtocol;
pub use shared::maintenance::ICacheCleanupProtocol;
pub use shared::maintenance::IDependencyReportProtocol;
pub use shared::maintenance::IDoctorProtocol;
pub use shared::maintenance::IMaintenanceAggregate;
pub use shared::maintenance::IProjectStatsProtocol;
pub use shared::maintenance::ISecurityScanProtocol;
pub use shared::maintenance::ISelfUpdateProtocol;
pub use shared::maintenance::IToolExecutorProtocol;
pub use shared::maintenance::IToolUpdateProtocol;
pub use shared::maintenance::IToolchainDiagnosticProtocol;

pub mod agent_maintenance_orchestrator;
pub use agent_maintenance_orchestrator::{MaintenanceCommandsOrchestrator, MaintenanceDeps};

pub mod capabilities_maintenance_checker;
pub use capabilities_maintenance_checker::MaintenanceChecker;

pub mod capabilities_tool_executor_adapter;
pub use capabilities_tool_executor_adapter::ToolExecutorAdapter;

pub mod root_maintenance_container;
