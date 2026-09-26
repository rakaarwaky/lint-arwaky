// external-lint — taxonomy types for adapter utilities
pub mod contract_external_lint_aggregate;
pub mod contract_external_lint_protocol;
pub mod taxonomy_external_lint_request_vo;
pub mod taxonomy_external_lint_vo;

// ─── Re-exports ────────────────────────────────────────────
// Barrel re-export pattern: allows consumers to import directly

// ── Contract traits ──
pub use contract_external_lint_aggregate::IExternalLintAggregate;
pub use contract_external_lint_protocol::ICommandExecutorProtocol;
pub use contract_external_lint_protocol::IExternalLintExecutorProtocol;
pub use contract_external_lint_protocol::IExternalLintSelectorProtocol;
pub use contract_external_lint_protocol::ILinterAdapterProtocol;

// ── Taxonomy VOs ──
pub use taxonomy_external_lint_request_vo::ExternalLintRequest;
pub use taxonomy_external_lint_request_vo::ExternalLintResponse;
pub use taxonomy_external_lint_vo::ExternalLintContext;
