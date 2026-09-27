// report-formatter — contract and taxonomy types
pub mod contract_report_formatter_aggregate;
pub mod contract_report_formatter_protocol;
pub mod taxonomy_report_vo;

// ─── Re-exports ────────────────────────────────────────────
// Barrel re-export pattern: allows consumers to import directly

// ── Contract traits ──
pub use contract_report_formatter_aggregate::IReportFormatterAggregate;
pub use contract_report_formatter_protocol::IReportFormatterProtocol;
pub use taxonomy_report_vo::JsonDiagnostic;
pub use taxonomy_report_vo::JsonReportDto;
pub use taxonomy_report_vo::JsonSummary;
pub use taxonomy_report_vo::JsonViolation;
pub use taxonomy_report_vo::SarifArtifactLocation;
pub use taxonomy_report_vo::SarifDriver;
pub use taxonomy_report_vo::SarifLocation;
pub use taxonomy_report_vo::SarifLog;
pub use taxonomy_report_vo::SarifMessage;
pub use taxonomy_report_vo::SarifPhysicalLocation;
pub use taxonomy_report_vo::SarifRegion;
pub use taxonomy_report_vo::SarifResult;
pub use taxonomy_report_vo::SarifRule;
pub use taxonomy_report_vo::SarifRun;
pub use taxonomy_report_vo::SarifTool;
