// PURPOSE: Supervisor feature contracts (AES102 `_protocol`) and taxonomy types
//
// Three capability seams for the supervisor workflow feature:
//   FR-001 GitHub issue discovery    → IIssueDiscoveryProtocol
//   FR-002 Orca worktree management  → IWorktreeManagerProtocol
//   FR-003 PR monitoring & merging   → IPrMonitorProtocol
//
// All types are pure taxonomy/contract: no business logic, no I/O.
pub mod contract_supervisor_aggregate;
pub mod contract_supervisor_protocol;
pub mod taxonomy_supervisor_request;
pub mod taxonomy_supervisor_response;
pub mod taxonomy_supervisor_vo;

// ─── Re-exports ────────────────────────────────────────────
// Barrel re-export pattern: allows consumers to import directly

// ── Contract traits ──
pub use contract_supervisor_aggregate::ISupervisorAggregate;
pub use contract_supervisor_protocol::IIssueDiscoveryProtocol;
pub use contract_supervisor_protocol::IPrMonitorProtocol;
pub use contract_supervisor_protocol::IWorktreeManagerProtocol;

// ── Request / response payloads (taxonomy layer) ──
pub use taxonomy_supervisor_request::SupervisorRequest;
pub use taxonomy_supervisor_response::SupervisorResponse;

// ── Taxonomy types ──
pub use taxonomy_supervisor_vo::CIStatus;
pub use taxonomy_supervisor_vo::GitHubIssueVo;
pub use taxonomy_supervisor_vo::PrInfo;
pub use taxonomy_supervisor_vo::SUP_VISIBILITY_MAX_CI_RETRIES;
pub use taxonomy_supervisor_vo::SubagentBrief;
pub use taxonomy_supervisor_vo::SupervisorCycleReport;
pub use taxonomy_supervisor_vo::SupervisorError;
pub use taxonomy_supervisor_vo::SupervisorIssueOutcome;
pub use taxonomy_supervisor_vo::WorktreeHandle;
