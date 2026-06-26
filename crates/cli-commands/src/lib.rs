// PURPOSE: Module declarations for cli-commands (surfaces, transport, container)
pub use shared::cli_commands::taxonomy_catalog_constant::COMMAND_CATALOG;
pub use shared::cli_commands::taxonomy_cli_vo::{get_cli, Cli, Commands};
pub use shared::cli_commands::taxonomy_command_catalog_vo::{command_catalog, CommandCatalogVO};
pub use shared::cli_commands::taxonomy_metadata_vo::CommandMetadataVO;

pub mod infrastructure_check_context;
pub mod surface_check_command;
pub use surface_check_command::CheckCommandsSurface;
pub mod surface_check_main;
pub mod surface_common_command;
pub use surface_common_command::CliLanguageDetector;
pub mod surface_fix_command;
pub use surface_fix_command::FixCommandsSurface;
pub mod surface_maintenance_command;
pub use surface_maintenance_command::MaintenanceCommandsSurface;
pub mod surface_git_command;
pub mod surface_plugin_command;
pub mod surface_setup_command;
pub mod surface_tui_command;
pub use surface_tui_command::TuiCommandSurface;
pub mod surface_watch_command;
pub use surface_watch_command::WatchCommandsSurface;
pub mod infrastructure_language_detector;
pub mod surface_config_command;
