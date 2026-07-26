pub mod contract_diff_protocol;
pub mod contract_git_hooks_aggregate;
pub mod contract_hook_protocol;
pub mod contract_manager_protocol;
pub mod contract_orchestrator_aggregate;
pub mod taxonomy_git_diff_data_vo;
pub mod taxonomy_hook_error;
pub mod utility_git_io;

// ─── Re-exports ────────────────────────────────────────────
// Barrel re-export pattern: allows consumers to import directly

// ── Contract traits ──
pub use contract_diff_protocol::IDiffProtocol;
pub use contract_git_hooks_aggregate::GitHooksAggregate;
pub use contract_hook_protocol::IHookProtocol;
pub use contract_manager_protocol::IHookManagerProtocol;
pub use contract_orchestrator_aggregate::HookManagementOrchestratorAggregate;

// ── Taxonomy types ──
pub use taxonomy_git_diff_data_vo::GitDiffDataVO;
pub use taxonomy_git_diff_data_vo::GitDiffSideVO;
pub use taxonomy_git_diff_data_vo::GitDiffStatus;
pub use taxonomy_git_diff_data_vo::HookIgnoreUpdateVO;
pub use taxonomy_hook_error::GitHookError;
