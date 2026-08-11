// PURPOSE: Dispatcher crate — Utility Surface
// Source of truth for shared scan/CI business logic
// CLI/MCP/TUI call these functions then format output themselves

pub mod surface_check_action;
pub mod surface_ci_action;
pub mod surface_config_action;
pub mod surface_external_action;
pub mod surface_fix_action;
pub mod surface_git_action;
pub mod surface_import_action;
pub mod surface_maintenance_action;
pub mod surface_naming_action;
pub mod surface_orphan_action;
pub mod surface_plugin_action;
pub mod surface_quality_action;
pub mod surface_role_action;
pub mod surface_setup_action;
pub mod surface_version_action;
pub mod surface_watch_action;
