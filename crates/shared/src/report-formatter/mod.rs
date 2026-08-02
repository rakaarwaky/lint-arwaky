// report-formatter — contract and taxonomy types
pub mod contract_report_formatter_aggregate;
pub mod contract_report_formatter_protocol;
pub mod taxonomy_json_dto_vo;
pub mod taxonomy_sarif_driver_vo;
pub mod taxonomy_sarif_location_vo;
pub mod taxonomy_sarif_log_vo;
pub mod taxonomy_sarif_result_vo;

// ─── Re-exports ────────────────────────────────────────────
// Barrel re-export pattern: allows consumers to import directly

// ── Contract traits ──
pub use contract_report_formatter_aggregate::IReportFormatterAggregate;
pub use contract_report_formatter_protocol::IReportFormatterProtocol;
pub use taxonomy_json_dto_vo::{JsonDiagnostic, JsonReportDto, JsonSummary, JsonViolation};
pub use taxonomy_sarif_driver_vo::{SarifDriver, SarifRule};
pub use taxonomy_sarif_location_vo::{
    SarifArtifactLocation, SarifLocation, SarifPhysicalLocation, SarifRegion,
};
pub use taxonomy_sarif_log_vo::{SarifLog, SarifRun, SarifTool};
pub use taxonomy_sarif_result_vo::{SarifMessage, SarifResult};
