// PURPOSE: Module declarations and re-exports for all CLI command surfaces and aggregates
pub mod agent_dev_orchestrator;
pub use agent_dev_orchestrator::DevCommandsOrchestrator;
pub mod agent_maintenance_orchestrator;
pub use agent_maintenance_orchestrator::MaintenanceCommandsOrchestrator;
pub mod contract_dev_aggregate;
pub use contract_dev_aggregate::DevCommandsAggregate;
pub mod taxonomy_command_catalog_vo;
pub use taxonomy_command_catalog_vo::{command_catalog, CommandCatalogVO};

pub mod contract_maintenance_aggregate;
pub use contract_maintenance_aggregate::MaintenanceCommandsAggregate;

pub mod surface_check_command;
pub use surface_check_command::{register_check_commands, CheckCommandsSurface};
pub mod surface_core_command;
pub use surface_core_command::{
    get_cli, get_surface, Cli, Commands, ConfigCommands, CoreCommandsSurface, SetupCommands,
};
pub mod surface_dev_command;
pub use surface_dev_command::{register_dev_commands, DevCommandsSurface};
pub mod surface_fix_command;
pub use surface_fix_command::{register_fix_commands, FixCommandsSurface};
pub mod surface_git_command;
pub use surface_git_command::{register_git_commands, GitCommandsSurface};
pub mod surface_bootstrap_command;
pub use surface_bootstrap_command::{run_cli_entry, BootstrapCommandSurface};
pub mod surface_maintenance_command;
pub use surface_maintenance_command::{register_maintenance_commands, MaintenanceCommandsSurface};
pub mod surface_multi_command;
pub use surface_multi_command::{register_multi_commands, MultiCommandsSurface};
pub mod surface_output_controller;
pub use surface_output_controller::{
    get_output_dir, set_container, tee_stdout, write_output, OutputControllerSurface,
};
pub mod surface_plugin_command;
pub use surface_plugin_command::{register_plugin_commands, PluginCommandsSurface};
pub mod surface_report_command;
pub use surface_report_command::{register_report_commands, ReportCommandsSurface};
pub mod surface_setup_command;
pub use surface_setup_command::{get_setup, register_setup_commands, SetupCommandsSurface};
pub mod surface_tui_command;
pub use surface_tui_command::TuiCommandSurface;

pub mod surface_watch_command;
pub use surface_watch_command::{register_watch_command, WatchCommandsSurface, WatchdogBridge};
pub mod surface_config_command;
pub use surface_config_command::handle_config;
pub mod surface_map_command;
pub use surface_map_command::{
    handle_cancel, handle_diff, handle_export, handle_import, handle_suggest,
};
// backend logic moved to code-analysis/capabilities_project_target_resolver.rs
pub use crate::code_analysis::{
    compute_score, count_loc, has_critical, lint_path, normalize_project_root, resolve_target,
    walk_rs_files,
};
pub mod taxonomy_catalog_constant;
pub use taxonomy_catalog_constant::COMMAND_CATALOG;
pub mod taxonomy_metadata_vo;
pub use taxonomy_metadata_vo::CommandMetadataVO;
