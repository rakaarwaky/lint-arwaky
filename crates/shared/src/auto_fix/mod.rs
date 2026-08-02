// auto-fix — taxonomy and contract types
pub mod contract_file_adapter_protocol;
pub mod contract_fix_aggregate;
pub mod contract_fix_protocol;
pub mod taxonomy_fix_applied_event;
pub mod taxonomy_fix_outcome_vo;
pub mod taxonomy_fix_vo;

// ─── Re-exports ────────────────────────────────────────────
// Barrel re-export pattern: allows consumers to import directly

// ── Contract traits ──
pub use contract_file_adapter_protocol::IFileAdapterProtocol;
pub use contract_fix_aggregate::LintFixOrchestratorAggregate;
pub use contract_fix_protocol::IFixProtocol;

// ── Taxonomy types ──
pub use taxonomy_fix_applied_event::FixApplied;
pub use taxonomy_fix_outcome_vo::{FailReason, FixOutcome, SkipReason};
pub use taxonomy_fix_vo::FixResult;
