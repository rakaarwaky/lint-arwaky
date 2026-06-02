"""Auto-Linter Surfaces Layer.

This package exposes user-facing interfaces:
- CLI entry point (``cli_main``) from cli_main
- Output utilities (get_output_dir, write_output, tee_stdout)
- MCP server entry point (mcp_server)
"""

# First import — triggers sys.path bootstrap as module-level side effect.
from . import syspath_bootstrap_handler as _bootstrap

from .cli_analysis_command import AnalysisCommandsSurface, register_analysis_commands
from .cli_check_command import CheckCommandsSurface, register_check_commands
from .cli_core_command import CoreCommandsSurface, get_cli, get_surface
from .cli_dev_command import DevCommandsSurface, register_dev_commands
from .cli_fix_command import FixCommandsSurface, register_fix_commands
from .cli_main_handler import MainHandlerSurface
from .cli_maintenance_command import MaintenanceCommandsSurface, register_maintenance_commands
from .cli_output_controller import (
    OutputControllerSurface,
    get_output_dir,
    write_output,
    tee_stdout,
)
from .cli_setup_command import SetupCommandsSurface, register_setup_commands
from .cli_setup_controller import SetupManagementSurface, register_setup_management
from .cli_watch_command import WatchCommandsSurface, WatchdogBridge, register_watch_command
from .core_git_command import GitCommandsSurface, register_git_commands
from .core_multi_command import MultiCommandsSurface, register_multi_commands
from .core_plugin_command import PluginCommandsSurface, register_plugin_commands
from .core_report_command import ReportCommandsSurface, register_report_commands
from .mcp_command_handler import McpCommandCatalogSurface
from .mcp_command_handler import register_catalog_commands
from .mcp_command_handler import register_list_commands
from .mcp_command_handler import register_read_skill_context
from .mcp_client_handler import McpDesktopClientSurface, register_desktop_client
from .mcp_execute_command import McpExecuteCommandSurface, register_execute_commands
from .mcp_health_handler import McpHealthCheckSurface, register_health_commands
from .mcp_job_handler import McpJobCommandsSurface, register_job_commands
from .mcp_job_handler import register_check_status, register_cancel_job
from .mcp_server_handler import McpServerHandlerSurface
from .mcp_tools_store import McpToolsRegistrySurface, register_tools

from ..contract import OutputClientAggregate

__all__ = [
    "AnalysisCommandsSurface",
    "register_analysis_commands",
    "CheckCommandsSurface",
    "register_check_commands",
    "CoreCommandsSurface",
    "get_cli",
    "get_surface",
    "DevCommandsSurface",
    "register_dev_commands",
    "FixCommandsSurface",
    "register_fix_commands",
    "MainHandlerSurface",
    "MaintenanceCommandsSurface",
    "register_maintenance_commands",
    "OutputControllerSurface",
    "get_output_dir",
    "write_output",
    "tee_stdout",
    "SetupCommandsSurface",
    "register_setup_commands",
    "SetupManagementSurface",
    "register_setup_management",
    "WatchCommandsSurface",
    "WatchdogBridge",
    "register_watch_command",
    "GitCommandsSurface",
    "register_git_commands",
    "MultiCommandsSurface",
    "register_multi_commands",
    "PluginCommandsSurface",
    "register_plugin_commands",
    "ReportCommandsSurface",
    "register_report_commands",
    "McpCommandCatalogSurface",
    "register_catalog_commands",
    "register_list_commands",
    "register_read_skill_context",
    "McpDesktopClientSurface",
    "register_desktop_client",
    "McpExecuteCommandSurface",
    "register_execute_commands",
    "McpHealthCheckSurface",
    "register_health_commands",
    "McpJobCommandsSurface",
    "register_job_commands",
    "register_check_status",
    "register_cancel_job",
    "McpServerHandlerSurface",
    "McpToolsRegistrySurface",
    "register_tools",
    "OutputClientAggregate",
    "_bootstrap",
]
