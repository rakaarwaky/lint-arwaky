// cli-commands — taxonomy and contract types
pub mod taxonomy_command_vo;
pub mod taxonomy_scan_request;
pub use crate::common::taxonomy_format_vo;
// Backward-compat alias: downstream crates (external-lint, etc.) import via
// `shared::cli_commands::taxonomy_result_vo::LintResult`. The alias points
// to the canonical module so both paths resolve to the same type.
pub use crate::common::taxonomy_lint_result_vo as taxonomy_result_vo;

// ─── Re-exports ────────────────────────────────────────────
// Barrel re-export pattern: allows consumers to import directly

// ── Taxonomy types ──
pub use crate::common::taxonomy_lint_result_vo::LintResult;
pub use crate::common::taxonomy_lint_result_vo::LintResultList;
pub use taxonomy_command_vo::COMMAND_CATALOG;
pub use taxonomy_command_vo::Cli;
pub use taxonomy_command_vo::CommandMetadataVO;
pub use taxonomy_command_vo::Commands;
pub use taxonomy_command_vo::DiagnosticSeverity;
pub use taxonomy_command_vo::PipelineDiagnostic;
pub use taxonomy_command_vo::PipelineError;
pub use taxonomy_command_vo::ScanReport;
pub use taxonomy_command_vo::TransportEndpoint;
pub use taxonomy_command_vo::TransportProtocol;
pub use taxonomy_command_vo::TransportUrlVO;
pub use taxonomy_command_vo::command_catalog;
pub use taxonomy_format_vo::Format;
pub use taxonomy_scan_request::ScanMode;
pub use taxonomy_scan_request::ScanRequest;
pub use taxonomy_scan_request::ScanTarget;
