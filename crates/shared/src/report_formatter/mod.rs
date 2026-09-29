// report-formatter — contract and taxonomy types
pub mod contract_report_formatter_aggregate;
pub mod contract_report_formatter_protocol;
pub mod taxonomy_report_formatter_vo;

// ─── Re-exports ────────────────────────────────────────────
// Barrel re-export pattern: allows consumers to import directly

// ── Contract traits ──
pub use contract_report_formatter_aggregate::IReportFormatterAggregate;
pub use contract_report_formatter_protocol::IJUnitFormatProtocol;
pub use contract_report_formatter_protocol::IJsonFormatProtocol;
pub use contract_report_formatter_protocol::ISarifFormatProtocol;
pub use contract_report_formatter_protocol::ITextFormatProtocol;
pub use taxonomy_report_formatter_vo::JsonDiagnostic;
pub use taxonomy_report_formatter_vo::JsonReportDto;
pub use taxonomy_report_formatter_vo::JsonSummary;
pub use taxonomy_report_formatter_vo::JsonViolation;
pub use taxonomy_report_formatter_vo::SarifArtifactLocation;
pub use taxonomy_report_formatter_vo::SarifDriver;
pub use taxonomy_report_formatter_vo::SarifLocation;
pub use taxonomy_report_formatter_vo::SarifLog;
pub use taxonomy_report_formatter_vo::SarifMessage;
pub use taxonomy_report_formatter_vo::SarifPhysicalLocation;
pub use taxonomy_report_formatter_vo::SarifRegion;
pub use taxonomy_report_formatter_vo::SarifResult;
pub use taxonomy_report_formatter_vo::SarifRule;
pub use taxonomy_report_formatter_vo::SarifRun;
pub use taxonomy_report_formatter_vo::SarifTool;
