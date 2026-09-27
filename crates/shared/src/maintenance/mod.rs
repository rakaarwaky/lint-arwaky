// maintenance — contract and taxonomy types
pub mod contract_maintenance_aggregate;
pub mod contract_maintenance_protocol;
pub mod taxonomy_maintenance_request;
pub mod taxonomy_maintenance_response;
pub mod taxonomy_maintenance_vo;

// ─── Re-exports ────────────────────────────────────────────
// Barrel re-export pattern: allows consumers to import directly

// ── Contract traits ──
pub use contract_maintenance_aggregate::IMaintenanceAggregate;
pub use contract_maintenance_protocol::IMaintenanceCheckerProtocol;
pub use contract_maintenance_protocol::IToolExecutorProtocol;
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
