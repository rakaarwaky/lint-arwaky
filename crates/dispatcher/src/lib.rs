// PURPOSE: Dispatcher crate — Utility Surface
// Source of truth for shared scan/CI business logic
// CLI/MCP/TUI call these functions then format output themselves

pub mod orchestrator_check_pipeline;
pub mod utility_pipeline_guard;
pub mod orchestrator_ci_pipeline;
pub mod orchestrator_config_pipeline;
pub mod orchestrator_docs_pipeline;
pub mod orchestrator_external_pipeline;
pub mod orchestrator_fix_pipeline;
pub mod orchestrator_git_pipeline;
pub mod orchestrator_import_pipeline;
pub mod orchestrator_maintenance_pipeline;
pub mod orchestrator_naming_pipeline;
pub mod orchestrator_orphan_pipeline;
pub mod orchestrator_plugin_pipeline;
pub mod orchestrator_quality_pipeline;
pub mod orchestrator_role_pipeline;
pub mod orchestrator_setup_pipeline;
pub mod orchestrator_structure_pipeline;
pub mod orchestrator_version_pipeline;
pub mod orchestrator_watch_pipeline;
