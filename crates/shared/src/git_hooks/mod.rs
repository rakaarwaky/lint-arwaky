pub mod contract_diff_protocol;
pub mod contract_git_hooks_aggregate;
pub mod contract_hook_protocol;
pub mod contract_manager_protocol;
pub mod taxonomy_git_diff_data_vo;
pub mod taxonomy_git_hooks_request_vo;
pub mod taxonomy_hook_error;

// ─── Re-exports ────────────────────────────────────────────
// Barrel re-export pattern: allows consumers to import directly

// ── Contract traits ──
pub use contract_diff_protocol::IDiffProtocol;
pub use contract_git_hooks_aggregate::IGitHooksAggregate;
pub use contract_hook_protocol::IHookProtocol;
pub use contract_manager_protocol::IHookManagerProtocol;

// ── Taxonomy types ──
pub use taxonomy_git_diff_data_vo::GitDiffDataVO;
pub use taxonomy_git_diff_data_vo::GitDiffSideVO;
pub use taxonomy_git_diff_data_vo::GitDiffStatus;
pub use taxonomy_git_diff_data_vo::HookIgnoreUpdateVO;
pub use taxonomy_git_hooks_request_vo::GitHooksRequest;
pub use taxonomy_git_hooks_request_vo::GitHooksResponse;
pub use taxonomy_hook_error::GitHookError;
