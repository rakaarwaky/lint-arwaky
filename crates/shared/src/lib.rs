// PURPOSE: shared — all taxonomy types, contract traits, and shared definitions
// No dependencies on other feature crates — this is the foundation layer.

#[path = "common/mod.rs"]
pub mod common;

// Re-export all taxonomy_* and contract_* types from common
// NOTE: widely used by downstream crates as shared::taxonomy_*. Do not remove.
pub use common::*;

#[path = "tui/mod.rs"]
pub mod tui;

// Feature-specific types (in feature folders)
#[path = "auto-fix/mod.rs"]
pub mod auto_fix;
#[path = "cli-commands/mod.rs"]
pub mod cli_commands;
#[path = "code-analysis/mod.rs"]
pub mod code_analysis;
#[path = "config-system/mod.rs"]
pub mod config_system;
#[path = "external-lint/mod.rs"]
pub mod external_lint;
#[path = "file-watch/mod.rs"]
pub mod file_watch;
#[path = "git-hooks/mod.rs"]
pub mod git_hooks;
#[path = "import-rules/mod.rs"]
pub mod import_rules;
#[path = "mcp-server/mod.rs"]
pub mod mcp_server;

#[path = "naming-rules/mod.rs"]
pub mod naming_rules;
#[path = "orphan-detector/mod.rs"]
pub mod orphan_detector;
#[path = "project-setup/mod.rs"]
pub mod project_setup;
#[path = "role-rules/mod.rs"]
pub mod role_rules;

#[path = "report-formatter/mod.rs"]
pub mod report_formatter;

#[path = "maintenance/mod.rs"]
pub mod maintenance;
