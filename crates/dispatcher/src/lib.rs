// PURPOSE: Dispatcher crate — Utility Surface
// Source of truth for shared scan/CI business logic
// CLI/MCP/TUI call these functions then format output themselves

pub mod surface_ci_action;
pub mod surface_external_action;
pub mod surface_import_action;
pub mod surface_naming_action;
pub mod surface_orphan_action;
pub mod surface_output_component;
pub mod surface_quality_action;
pub mod surface_role_action;
