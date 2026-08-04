// external-lint — taxonomy types for adapter utilities
pub mod contract_adapter_protocol;
pub mod contract_executor_protocol;
pub mod contract_external_lint_aggregate;
pub mod contract_external_lint_executor_protocol;
pub mod contract_external_lint_selector_protocol;
pub mod taxonomy_external_lint_vo;

// ─── Re-exports ────────────────────────────────────────────
// Barrel re-export pattern: allows consumers to import directly

// ── Contract traits ──
pub use contract_adapter_protocol::ILinterAdapterProtocol;
pub use contract_executor_protocol::ICommandExecutorProtocol;
pub use contract_external_lint_aggregate::IExternalLintAggregate;
pub use contract_external_lint_executor_protocol::IExternalLintExecutorProtocol;
pub use contract_external_lint_selector_protocol::IExternalLintSelectorProtocol;

// ── Taxonomy VOs ──
pub use taxonomy_external_lint_vo::ExternalLintContext;
