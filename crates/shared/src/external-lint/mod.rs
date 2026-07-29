// external-lint — taxonomy types for adapter utilities
pub mod contract_external_lint_aggregate;
pub mod contract_external_lint_executor_protocol;
pub mod contract_external_lint_selector_protocol;
pub mod contract_external_lint_utility_protocol;
pub mod utility_external_lint;
pub mod utility_external_lint_io;

// ─── Re-exports ────────────────────────────────────────────
// Barrel re-export pattern: allows consumers to import directly

// ── Contract traits ──
pub use contract_external_lint_aggregate::IExternalLintAggregate;
pub use contract_external_lint_executor_protocol::IExternalLintExecutorProtocol;
pub use contract_external_lint_selector_protocol::IExternalLintSelectorProtocol;
pub use contract_external_lint_utility_protocol::IExternalLintCargoProtocol;
pub use contract_external_lint_utility_protocol::IExternalLintCommandProtocol;
pub use contract_external_lint_utility_protocol::IExternalLintJsProtocol;
pub use contract_external_lint_utility_protocol::IExternalLintLanguageProtocol;
pub use contract_external_lint_utility_protocol::IExternalLintPathProtocol;
