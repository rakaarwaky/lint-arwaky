/// Lint Arwaky — architecture linter for Rust, Python, and TypeScript.
///
/// Aggregates every workspace crate and re-exports them for binaries
/// (`lint-arwaky-cli`, `lint-arwaky-mcp`, `lint-arwaky-tui`).
pub use auto_fix;
pub use cli_commands;
pub use config_system;
pub use dispatcher;
pub use external_lint;
pub use file_watch;
pub use filesystem;
pub use git_hooks;
pub use import_rules;
pub use maintenance;
pub use mcp_server;
pub use naming_rules;
pub use orphan_rules;
pub use project_setup;
pub use quality_rules;
pub use report_formatter;
pub use role_rules;
pub use shared_auto_fix;
pub use shared_cli_commands;
pub use shared_common;
pub use shared_config_system;
pub use shared_doc_rules;
pub use shared_external_lint;
pub use shared_file_watch;
pub use shared_filesystem;
pub use shared_git_hooks;
pub use shared_import_rules;
pub use shared_maintenance;
pub use shared_mcp_server;
pub use shared_naming_rules;
pub use shared_orphan_rules;
pub use shared_project_setup;
pub use shared_quality_rules;
pub use shared_report_formatter;
pub use shared_role_rules;
pub use shared_structure_rules;
pub use shared_tui;
pub use tui;

// ── Root entry wiring ──────────────────────────────────────
pub mod root_entry_container;
