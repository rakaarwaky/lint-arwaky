// PURPOSE: git_hooks — feature crate for Git hook management and diff analysis

// ── Capabilities (concrete implementations) ──
pub mod capabilities_diff_checker;
pub mod capabilities_hook_adapter;
pub mod capabilities_hook_manager;

// ── Agent (orchestration) ──
pub mod agent_git_hooks_orchestrator;

// ── Root (composition, wiring) ──
pub mod root_git_hooks_container;
