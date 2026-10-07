// PURPOSE: shared-logging — taxonomy and contract types for the logging
// feature (real-time scan feed: filter, subscriber install, phase timing,
// walker reporting).
//
// LogVerbosity lives in shared-common (it's a true common taxonomy type
// used by the CLI entry point and every capability). This crate owns:
//   • the four capability protocols
//   • the single aggregate trait
//   • PhaseTimer + SkipReason (feature-specific taxonomy)
//   • AUDIT_TARGET (feature-specific constant)

pub mod contract_logging_aggregate;
pub mod contract_logging_protocol;
pub mod taxonomy_logging_vo;

// ─── Re-exports ────────────────────────────────────────────

// Contract traits
pub use contract_logging_aggregate::ILoggingAggregate;
pub use contract_logging_protocol::{
    IFilterBuildProtocol, IPhaseTimerProtocol, ISubscriberInstallProtocol, IWalkerReportProtocol,
};
// Backward-compat alias: the old single-trait seam kept as a type alias so
// that existing code importing ISubscriberInitProtocol continues to resolve.
pub use contract_logging_protocol::ISubscriberInstallProtocol as ISubscriberInitProtocol;

// Taxonomy types
pub use taxonomy_logging_vo::{AUDIT_TARGET, PhaseTimer, SkipReason, WalkAction};

// Re-export LogVerbosity from shared-common for convenience.
pub use shared_common::LogVerbosity;
