// PURPOSE: git_hooks — lightweight module export for GitHooksOrchestrator (git-hooks feature)
pub use shared::git_hooks::contract_diff_protocol::IDiffProtocol;
pub use shared::git_hooks::contract_git_hooks_aggregate::GitHooksAggregate;
pub use shared::git_hooks::contract_hook_protocol::IHookProtocol;
pub use shared::git_hooks::contract_manager_port::IHookManagerPort;
pub use shared::git_hooks::taxonomy_diff_result_vo::GitDiffResultVO;
pub use shared::git_hooks::taxonomy_hook_error::GitHookError;
pub mod agent_git_hooks_orchestrator;
pub use agent_git_hooks_orchestrator::GitHooksOrchestrator;
pub mod capabilities_diff_checker;
pub use capabilities_diff_checker::DiffChecker;
pub mod capabilities_hook_manager;
pub use capabilities_hook_manager::HookManager;
pub mod infrastructure_file_system_check_adapter;
pub mod infrastructure_git_command_adapter;
pub mod infrastructure_hook_adapter;
pub use infrastructure_hook_adapter::GitHookAdapter;
pub mod root_git_hooks_container;
