pub mod contract_git_hooks_aggregate;
pub mod contract_git_hooks_protocol;
pub mod taxonomy_git_hooks_constant;
pub mod taxonomy_git_hooks_error;
pub mod taxonomy_git_hooks_request;
pub mod taxonomy_git_hooks_response;
pub mod taxonomy_git_hooks_vo;

// ─── Re-exports ────────────────────────────────────────────
// Barrel re-export pattern: allows consumers to import directly

// ── Contract traits ──
pub use contract_git_hooks_aggregate::IGitHooksAggregate;
pub use contract_git_hooks_protocol::IConfigInitProtocol;
pub use contract_git_hooks_protocol::IDiffDataProtocol;
pub use contract_git_hooks_protocol::IDiffDetectionProtocol;
pub use contract_git_hooks_protocol::IHookCheckProtocol;
pub use contract_git_hooks_protocol::IHookInstallProtocol;
pub use contract_git_hooks_protocol::IHookUninstallProtocol;
pub use contract_git_hooks_protocol::IIgnoreRuleProtocol;

// ── Taxonomy types ──
pub use taxonomy_git_hooks_constant::LINTABLE_EXTENSIONS;
pub use taxonomy_git_hooks_error::GitHookError;
pub use taxonomy_git_hooks_request::GitHooksRequest;
pub use taxonomy_git_hooks_response::GitHooksResponse;
pub use taxonomy_git_hooks_vo::GitDiffDataVO;
pub use taxonomy_git_hooks_vo::GitDiffSideVO;
pub use taxonomy_git_hooks_vo::GitDiffStatus;
pub use taxonomy_git_hooks_vo::HookIgnoreUpdateVO;
