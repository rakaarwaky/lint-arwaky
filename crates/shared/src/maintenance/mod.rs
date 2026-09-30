// maintenance — contract and taxonomy types
pub mod contract_maintenance_aggregate;
pub mod contract_maintenance_protocol;
pub mod taxonomy_maintenance_constant;
pub mod taxonomy_maintenance_request;
pub mod taxonomy_maintenance_response;
pub mod taxonomy_maintenance_vo;
pub mod utility_maintenance_helpers;
pub mod utility_tool_executor;

// ─── Re-exports ────────────────────────────────────────────
// Barrel re-export pattern: allows consumers to import directly

// ── Contract traits ──
pub use contract_maintenance_aggregate::IMaintenanceAggregate;
pub use contract_maintenance_protocol::IAdapterHealthProtocol;
pub use contract_maintenance_protocol::ICacheCleanupProtocol;
pub use contract_maintenance_protocol::IDependencyReportProtocol;
pub use contract_maintenance_protocol::IDoctorProtocol;
pub use contract_maintenance_protocol::IProjectStatsProtocol;
pub use contract_maintenance_protocol::ISecurityScanProtocol;
pub use contract_maintenance_protocol::ISelfUpdateProtocol;
pub use contract_maintenance_protocol::IToolUpdateProtocol;

pub use taxonomy_maintenance_constant::GITHUB_REPO;
pub use taxonomy_maintenance_request::MaintenanceRequest;
pub use taxonomy_maintenance_response::MaintenanceResponse;

// ── Taxonomy types ──
pub use taxonomy_maintenance_vo::DependencyInfo;
pub use taxonomy_maintenance_vo::DependencyReport;
pub use taxonomy_maintenance_vo::DoctorResultVO;
pub use taxonomy_maintenance_vo::HealthCheckAdapterVO;
pub use taxonomy_maintenance_vo::HealthCheckResult;
pub use taxonomy_maintenance_vo::MaintenanceStatsVO;
pub use taxonomy_maintenance_vo::SecurityFinding;
pub use taxonomy_maintenance_vo::SecurityScanReport;
pub use taxonomy_maintenance_vo::SelfUpdateResultVO;
pub use taxonomy_maintenance_vo::ToolOutput;
pub use taxonomy_maintenance_vo::ToolStatus;
pub use taxonomy_maintenance_vo::ToolchainDiagnostics;
