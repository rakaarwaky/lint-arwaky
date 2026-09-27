// external-lint — taxonomy types for adapter utilities
pub mod contract_external_lint_aggregate;
pub mod contract_external_lint_protocol;
pub mod taxonomy_external_lint_request;
pub mod taxonomy_external_lint_response;
pub mod taxonomy_external_lint_vo;

// ─── Re-exports ────────────────────────────────────────────
// Barrel re-export pattern: allows consumers to import directly

// ── Contract traits ──
pub use contract_external_lint_aggregate::IExternalLintAggregate;
pub use contract_external_lint_protocol::IAdapterScanProtocol;
pub use contract_external_lint_protocol::ICargoDirProtocol;
pub use contract_external_lint_protocol::ICommandExecutorProtocol;
pub use contract_external_lint_protocol::IExternalLintSelectorProtocol;
pub use contract_external_lint_protocol::IJsToolResolutionProtocol;
pub use contract_external_lint_protocol::ILanguageDetectProtocol;
pub use contract_external_lint_protocol::ILinterAdapterProtocol;
pub use contract_external_lint_protocol::INormalizeProtocol;

// ── Taxonomy VOs ──
pub use taxonomy_external_lint_request::ExternalLintRequest;
pub use taxonomy_external_lint_response::ExternalLintResponse;
pub use taxonomy_external_lint_vo::ExternalLintContext;
