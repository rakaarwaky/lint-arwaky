// PURPOSE: Library root — re-exports all workspace member crates for CLI/MCP binary targets
pub use auto_fix;
pub use cli_commands;
pub use code_analysis;
pub use config_system;
pub use file_system;
pub use file_watch;
pub use git_hooks;
pub use import_rules;
pub use language_adapters;
pub use lifecycle_state;
pub use mcp_server;
pub use metrics_service;
pub use multi_project;
pub use naming_rules;
pub use orphan_detector;
pub use output_report;
pub use pipeline_jobs;
pub use plugin_system;
pub use project_setup;
pub use role_rules;
pub use shared_common;
pub use source_parsing;

// Root layer modules (entry points + composition root)
pub mod root_cli_main_entry;
pub mod root_composition_container;
pub mod root_mcp_main_entry;
pub mod root_tui_main_entry;
