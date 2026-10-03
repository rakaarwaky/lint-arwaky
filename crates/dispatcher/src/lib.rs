// PURPOSE: #570 — dispatcher surface-orchestrator re-export shim.
//
// The `surface_*_action` modules have moved to `shared-structure-rules`
// (issue #570: surface crates must not depend on the dispatcher crate).
// This crate re-exports every module so existing import paths
// (`dispatcher::surface_check_action::...`, `dispatcher_lint_arwaky::...`)
// keep resolving. Surfaces (cli-commands, mcp-server, tui) should import
// from `shared_structure_rules::surface_*` directly.
pub use shared_structure_rules::surface_check_action;
pub use shared_structure_rules::surface_ci_action;
pub use shared_structure_rules::surface_config_action;
pub use shared_structure_rules::surface_docs_action;
pub use shared_structure_rules::surface_external_action;
pub use shared_structure_rules::surface_fix_action;
pub use shared_structure_rules::surface_git_action;
pub use shared_structure_rules::surface_import_action;
pub use shared_structure_rules::surface_layer_scan_action;
pub use shared_structure_rules::surface_maintenance_action;
pub use shared_structure_rules::surface_naming_action;
pub use shared_structure_rules::surface_orphan_action;
pub use shared_structure_rules::surface_plugin_action;
pub use shared_structure_rules::surface_quality_action;
pub use shared_structure_rules::surface_role_action;
pub use shared_structure_rules::surface_setup_action;
pub use shared_structure_rules::surface_structure_action;
pub use shared_structure_rules::surface_test_entries;
pub use shared_structure_rules::surface_version_action;
pub use shared_structure_rules::surface_watch_action;
