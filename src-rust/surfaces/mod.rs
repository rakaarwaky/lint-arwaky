//! # Surfaces Layer — The User-Facing Boundary
//!
//! This module is the **outermost layer** of the AES architecture. It contains all
//! user-facing entry points: CLI commands, MCP protocol handlers, and system
//! bootstrap interfaces. Every surface delegates to the Agent layer for orchestration.
//!
//! ## Layer Rules (AES Compliance)
//! - **Allowed Imports**: All layers (`taxonomy`, `contract`, `capabilities`,
//!   `infrastructure`, `agent`, `surfaces`).
//! - **Allowed Suffixes**: `_command`, `_handler`, `_controller`, `_surface`
//! - **Responsibility**: User interaction only — parse input, call orchestrators,
//!   format output.
//!
//! ## Module Index
//!
//! | Domain / Feature | Key Types | Description |
//! |------------------|-----------|-------------|
//! | **CLI Core** | `Cli`, `Commands`, `CoreCommandsSurface`, `MainHandlerSurface` | CLI entry point & base dispatching |
//! | **CLI Check** | `CheckCommandsSurface` | Lint checking command surface |
//! | **CLI Analysis** | `AnalysisCommandsSurface` | Architecture analysis surface |
//! | **CLI Fix** | `FixCommandsSurface` | Auto-fix command surface |
//! | **CLI Watch** | `WatchCommandsSurface`, `WatchdogBridge` | File watch command surface |
//! | **CLI Dev** | `DevCommandsSurface` | Developer utilities surface |
//! | **CLI Setup** | `SetupCommandsSurface`, `SetupManagementSurface` | Configuration & MCP setup surface |
//! | **CLI Maintenance** | `MaintenanceCommandsSurface` | System maintenance surface |
//! | **CLI Output** | `OutputControllerSurface` | Output formatting & file writing |
//! | **Git** | `GitCommandsSurface` | Git integration command surface |
//! | **Multi-Project** | `MultiCommandsSurface` | Multi-project command surface |
//! | **Plugin** | `PluginCommandsSurface` | Plugin management surface |
//! | **Report** | `ReportCommandsSurface` | Report generation surface |
//! | **MCP Server** | `McpServerHandlerSurface`, `McpToolsHandler` | MCP protocol server handling |
//! | **MCP Health** | `McpHealthCheckSurface` | MCP server health check |
//! | **MCP Jobs** | `McpJobCommandsSurface` | MCP background job management |
//! | **MCP Commands** | `McpCommandCatalogSurface`, `CommandEntry` | MCP command catalog & listing |
//! | **MCP Execute** | `RUNNING_JOBS` | MCP command execution handler |
//! | **MCP Client** | `McpDesktopClientSurface` | Desktop client configuration |
//! | **Bootstrap** | `SyspathBootstrapHandler` | System path bootstrapping |

pub mod cli_analysis_command;
pub mod cli_check_command;
pub mod cli_core_command;
pub mod cli_dev_command;
pub mod cli_fix_command;
pub mod cli_main_handler;
pub mod cli_maintenance_command;
pub mod cli_output_controller;
pub mod cli_setup_command;
pub mod cli_setup_controller;
pub mod cli_watch_command;
pub mod core_git_command;
pub mod core_multi_command;
pub mod core_plugin_command;
pub mod core_report_command;
pub mod mcp_client_handler;
pub mod mcp_command_handler;
pub mod mcp_execute_command;
pub mod mcp_health_handler;
pub mod mcp_job_handler;
pub mod mcp_server_handler;
pub mod mcp_tools_command;
pub mod mcp_tools_handler;
pub mod syspath_bootstrap_handler;

// ═══════════════════════════════════════════════════════════════════════════════
// PUBLIC RE-EXPORTS (Flat Access via Barrel)
// ═══════════════════════════════════════════════════════════════════════════════

pub use cli_analysis_command::{register_analysis_commands, AnalysisCommandsSurface};
pub use cli_check_command::{register_check_commands, CheckCommandsSurface};
pub use cli_core_command::{
    get_cli, get_surface, Cli, Commands, ConfigCommands, CoreCommandsSurface, SetupCommands,
};
pub use cli_dev_command::DevCommandsSurface;
pub use cli_fix_command::{register_fix_commands, FixCommandsSurface};
pub use cli_main_handler::{run_cli_entry, MainHandlerSurface};
pub use cli_maintenance_command::{register_maintenance_commands, MaintenanceCommandsSurface};
pub use cli_output_controller::{
    get_output_dir, set_container, tee_stdout, write_output, OutputControllerSurface,
};
pub use cli_setup_command::{get_setup, register_setup_commands, SetupCommandsSurface};
pub use cli_setup_controller::{
    generate_env, generate_mcp_config, mcp_config_claude, mcp_config_hermes, mcp_config_vscode,
    register_setup_management, SetupManagementSurface,
};
pub use cli_watch_command::{register_watch_command, WatchCommandsSurface, WatchdogBridge};
pub use core_git_command::{register_git_commands, GitCommandsSurface};
pub use core_multi_command::{register_multi_commands, MultiCommandsSurface};
pub use core_plugin_command::{register_plugin_commands, PluginCommandsSurface};
pub use core_report_command::{register_report_commands, ReportCommandsSurface};
pub use mcp_client_handler::{register_desktop_client, McpDesktopClientSurface};
pub use mcp_command_handler::{
    list_commands_func, register_catalog_commands, register_list_commands,
    register_read_skill_context, CommandEntry, McpCommandCatalogSurface,
};
pub use mcp_execute_command::{register_execute_commands, RUNNING_JOBS};
pub use mcp_health_handler::{register_health_commands, McpHealthCheckSurface};
pub use mcp_job_handler::McpJobCommandsSurface;
pub use mcp_server_handler::McpServerHandlerSurface;
pub use mcp_tools_command::{
    commands_schema_tool, health_check_tool, list_commands_tool, read_skill_context_tool,
};
pub use mcp_tools_handler::register_tools;
pub use syspath_bootstrap_handler::SyspathBootstrapHandler;
