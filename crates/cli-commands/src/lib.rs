// PURPOSE: Module declarations for cli-commands (surfaces, transport, container)
pub use shared::cli_commands::taxonomy_catalog_constant::COMMAND_CATALOG;
pub use shared::cli_commands::taxonomy_command_catalog_vo::{command_catalog, CommandCatalogVO};
pub use shared::cli_commands::taxonomy_metadata_vo::CommandMetadataVO;

pub mod surface_check_command;
pub use surface_check_command::CheckCommandsSurface;
pub mod surface_core_command;
pub use surface_core_command::{
    get_cli, get_surface, Cli, Commands, ConfigCommands, CoreCommandsSurface, SetupCommands,
};
pub mod surface_dev_command;
pub use surface_dev_command::DevCommandsSurface;
pub mod surface_fix_command;
pub use surface_fix_command::FixCommandsSurface;
pub mod surface_git_command;
pub use surface_git_command::GitCommandsSurface;
pub mod surface_bootstrap_command;
pub use surface_bootstrap_command::{run_cli_entry, BootstrapCommandSurface};
pub mod surface_maintenance_command;
pub use surface_maintenance_command::MaintenanceCommandsSurface;
pub mod surface_multi_command;
pub use surface_multi_command::MultiCommandsSurface;
pub mod surface_output_controller;
pub use surface_output_controller::{
    get_output_dir, tee_stdout, write_output, OutputControllerSurface,
};
pub mod surface_plugin_command;
pub mod surface_report_command;
pub use surface_report_command::handle_report;
pub mod surface_setup_command;
pub use surface_setup_command::{get_setup, register_setup_commands, SetupCommandsSurface};
pub mod surface_tui_command;
pub use surface_tui_command::TuiCommandSurface;
pub mod surface_watch_command;
pub use surface_watch_command::{WatchCommandsSurface, WatchdogBridge};
pub mod surface_config_command;
pub use surface_config_command::handle_config;
pub mod surface_map_command;
pub use surface_map_command::{
    handle_cancel, handle_diff, handle_export, handle_import, handle_suggest,
};
