// auto-fix — taxonomy and contract types
pub mod contract_fix_aggregate;
pub mod contract_fix_protocol;
pub mod taxonomy_fix_applied_event;
pub mod taxonomy_fix_request;
pub mod taxonomy_fix_response;
pub mod taxonomy_fix_vo;

// ─── Re-exports ────────────────────────────────────────────
// Barrel re-export pattern: allows consumers to import directly

// ── Contract traits ──
pub use contract_fix_aggregate::IFixAggregate;
pub use contract_fix_protocol::IBypassFixProtocol;
pub use contract_fix_protocol::IFileAdapterProtocol;
pub use contract_fix_protocol::IFixPipelineProtocol;
pub use contract_fix_protocol::IManualReportProtocol;
pub use contract_fix_protocol::ISymbolRenameProtocol;
pub use contract_fix_protocol::IUnusedImportFixProtocol;

// ── Taxonomy types ──
pub use taxonomy_fix_applied_event::FixApplied;
pub use taxonomy_fix_request::FixRequest;
pub use taxonomy_fix_response::FixResponse;
pub use taxonomy_fix_vo::FixResult;
pub use taxonomy_fix_vo::{FailReason, FixOutcome, SkipReason};
