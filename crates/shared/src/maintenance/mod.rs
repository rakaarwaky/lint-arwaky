pub mod contract_maintenance_aggregate;
pub mod contract_maintenance_protocol;
pub mod taxonomy_doctor_vo;
pub mod taxonomy_stats_vo;
pub mod utility_dependency_io;

// ─── Re-exports ────────────────────────────────────────────
// Barrel re-export pattern: allows consumers to import directly

// ── Contract traits ──
pub use contract_maintenance_aggregate::MaintenanceCommandsAggregate;
pub use contract_maintenance_protocol::IMaintenanceCheckerProtocol;

// ── Taxonomy types ──
pub use taxonomy_doctor_vo::DependencyInfo;
pub use taxonomy_doctor_vo::DependencyReport;
pub use taxonomy_doctor_vo::DoctorResultVO;
pub use taxonomy_doctor_vo::SecurityFinding;
pub use taxonomy_doctor_vo::SecurityScanReport;
pub use taxonomy_doctor_vo::ToolStatus;
pub use taxonomy_doctor_vo::ToolchainDiagnostics;
pub use taxonomy_stats_vo::MaintenanceStatsVO;
