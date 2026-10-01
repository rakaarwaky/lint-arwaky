pub use shared_maintenance::IAdapterHealthProtocol;
pub use shared_maintenance::ICacheCleanupProtocol;
pub use shared_maintenance::IDependencyReportProtocol;
pub use shared_maintenance::IDoctorProtocol;
pub use shared_maintenance::IMaintenanceAggregate;
pub use shared_maintenance::IProjectStatsProtocol;
pub use shared_maintenance::ISecurityScanProtocol;
pub use shared_maintenance::ISelfUpdateProtocol;
pub use shared_maintenance::IToolUpdateProtocol;

pub mod agent_maintenance_orchestrator;
pub use agent_maintenance_orchestrator::{MaintenanceCommandsOrchestrator, MaintenanceDeps};

pub mod capabilities_project_stats_checker;
pub use capabilities_project_stats_checker::ProjectStatsChecker;
pub mod capabilities_cache_cleanup_checker;
pub use capabilities_cache_cleanup_checker::CacheCleanupChecker;
pub mod capabilities_tool_update_checker;
pub use capabilities_tool_update_checker::ToolUpdateChecker;
pub mod capabilities_doctor_checker;
pub use capabilities_doctor_checker::DoctorChecker;
pub mod capabilities_adapter_health_checker;
pub use capabilities_adapter_health_checker::AdapterHealthChecker;
pub mod capabilities_security_scan_checker;
pub use capabilities_security_scan_checker::SecurityScanChecker;
pub mod capabilities_dependency_report_checker;
pub use capabilities_dependency_report_checker::DependencyReportChecker;
pub mod capabilities_self_update_checker;
pub use capabilities_self_update_checker::SelfUpdateChecker;

pub mod root_maintenance_container;
