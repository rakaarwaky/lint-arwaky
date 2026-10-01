// cli-commands — taxonomy and contract types
pub mod taxonomy_cli_commands_request;
pub mod taxonomy_cli_commands_vo;
pub use shared_common::taxonomy_format_vo;
// Backward-compat alias: downstream crates (external-lint, etc.) import via
// `shared::cli_commands::taxonomy_result_vo::LintResult`. The alias points
// to the canonical module so both paths resolve to the same type.
pub use shared_common::taxonomy_lint_result_vo as taxonomy_result_vo;

// ─── Re-exports ────────────────────────────────────────────
// Barrel re-export pattern: allows consumers to import directly

// ── Taxonomy types ──
pub use shared_common::taxonomy_lint_result_vo::LintResult;
pub use shared_common::taxonomy_lint_result_vo::LintResultList;
pub use taxonomy_cli_commands_request::ScanMode;
pub use taxonomy_cli_commands_request::ScanRequest;
pub use taxonomy_cli_commands_request::ScanTarget;
pub use taxonomy_cli_commands_vo::COMMAND_CATALOG;
pub use taxonomy_cli_commands_vo::Cli;
pub use taxonomy_cli_commands_vo::CommandMetadataVO;
pub use taxonomy_cli_commands_vo::Commands;
pub use taxonomy_cli_commands_vo::DiagnosticSeverity;
pub use taxonomy_cli_commands_vo::PipelineDiagnostic;
pub use taxonomy_cli_commands_vo::PipelineError;
pub use taxonomy_cli_commands_vo::ScanReport;
pub use taxonomy_cli_commands_vo::TransportEndpoint;
pub use taxonomy_cli_commands_vo::TransportProtocol;
pub use taxonomy_cli_commands_vo::TransportUrlVO;
pub use taxonomy_cli_commands_vo::command_catalog;
pub use taxonomy_format_vo::Format;
