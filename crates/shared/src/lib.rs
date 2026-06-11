// PURPOSE: shared — all taxonomy types, contract traits, and shared definitions
// No dependencies on other feature crates — this is the foundation layer.

#[path = "common/mod.rs"]
pub mod common;

// Re-export all taxonomy_* and contract_* types from common
pub use common::*;

// Feature-specific types (in feature folders)
#[path = "auto-fix/mod.rs"]
pub mod auto_fix;
#[path = "cli-commands/mod.rs"]
pub mod cli_commands;
#[path = "code-analysis/mod.rs"]
pub mod code_analysis;
#[path = "config-system/mod.rs"]
pub mod config_system;
#[path = "file-system/mod.rs"]
pub mod file_system;
#[path = "file-watch/mod.rs"]
pub mod file_watch;
#[path = "git-hooks/mod.rs"]
pub mod git_hooks;
#[path = "import-rules/mod.rs"]
pub mod import_rules;
#[path = "language-adapters/mod.rs"]
pub mod language_adapters;
#[path = "lifecycle-state/mod.rs"]
pub mod lifecycle_state;
#[path = "mcp-server/mod.rs"]
pub mod mcp_server;
#[path = "metrics-service/mod.rs"]
pub mod metrics_service;
#[path = "multi-project/mod.rs"]
pub mod multi_project;
#[path = "naming-rules/mod.rs"]
pub mod naming_rules;
#[path = "orphan-detector/mod.rs"]
pub mod orphan_detector;
#[path = "output-report/mod.rs"]
pub mod output_report;
#[path = "pipeline-jobs/mod.rs"]
pub mod pipeline_jobs;
#[path = "plugin-system/mod.rs"]
pub mod plugin_system;
#[path = "project-setup/mod.rs"]
pub mod project_setup;
#[path = "role-rules/mod.rs"]
pub mod role_rules;
#[path = "source-parsing/mod.rs"]
pub mod source_parsing;