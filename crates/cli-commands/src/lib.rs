// PURPOSE: Module declarations for cli-commands (surfaces, transport, container)
pub use shared::cli_commands::taxonomy_catalog_constant::COMMAND_CATALOG;
pub use shared::cli_commands::taxonomy_command_catalog_vo::{command_catalog, CommandCatalogVO};
pub use shared::cli_commands::taxonomy_metadata_vo::CommandMetadataVO;

pub mod infrastructure_transport_client;
pub use infrastructure_transport_client::StdioClient;
pub mod root_transport_container;
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
