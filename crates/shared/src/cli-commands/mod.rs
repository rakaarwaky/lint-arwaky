// cli-commands — taxonomy and contract types
pub mod taxonomy_cli_vo;
pub mod taxonomy_command_catalog_vo;
pub mod taxonomy_format_vo;
pub mod taxonomy_position_vo;
pub mod taxonomy_protocol_vo;
pub mod taxonomy_result_vo;
pub mod taxonomy_scan_report_vo;
pub mod taxonomy_scan_request_vo;

// ─── Re-exports ────────────────────────────────────────────
// Barrel re-export pattern: allows consumers to import directly

// ── Taxonomy types ──
pub use taxonomy_cli_vo::Cli;
pub use taxonomy_cli_vo::Commands;
pub use taxonomy_command_catalog_vo::CommandCatalogVO;
pub use taxonomy_command_catalog_vo::CommandMetadataVO;
pub use taxonomy_format_vo::Format;
pub use taxonomy_position_vo::Position;
pub use taxonomy_protocol_vo::TransportEndpoint;
pub use taxonomy_protocol_vo::TransportProtocol;
pub use taxonomy_protocol_vo::TransportUrlVO;
pub use taxonomy_result_vo::LintResult;
pub use taxonomy_result_vo::LintResultList;
pub use taxonomy_scan_report_vo::DiagnosticSeverity;
pub use taxonomy_scan_report_vo::PipelineDiagnostic;
pub use taxonomy_scan_report_vo::PipelineError;
pub use taxonomy_scan_report_vo::ScanReport;
pub use taxonomy_scan_request_vo::ScanMode;
pub use taxonomy_scan_request_vo::ScanRequest;
pub use taxonomy_scan_request_vo::ScanTarget;
