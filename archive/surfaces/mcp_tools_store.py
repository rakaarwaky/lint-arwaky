"""
MCP Tools Registry - Bridges Capabilities to the Surface Layer.

Split into granular modules:
- mcp_desktop_client_handler.py - _get_client, _execute_with_retry, _running_jobs
- mcp_command_handler.py - list_commands, read_skill_context
- mcp_job_commands_handler.py - check_status, cancel_job
- mcp_health_handler.py   - health_check
- mcp_execute_command.py - execute_command tool
"""

from mcp.server.fastmcp import FastMCP

from ..contract import ServiceContainerAggregate


class McpToolsRegistrySurface:
    """Surface for bridging Capabilities to the MCP Layer."""

    def __init__(self, mcp: FastMCP):
        self.mcp = mcp

    def register_tools(self, container: ServiceContainerAggregate) -> None:
        """Bridges Capabilities to the MCP Surface (Domain 5)."""
        # Import and delegate to split modules
        from .mcp_execute_command import register_execute_commands
        from .mcp_command_handler import register_catalog_commands
        from .mcp_health_handler import register_health_commands
        from .mcp_client_handler import register_desktop_client

        register_execute_commands(self.mcp, container)
        register_catalog_commands(self.mcp, container)
        register_health_commands(self.mcp, container)
        register_desktop_client(self.mcp, container)


def register_tools(mcp: FastMCP, container: ServiceContainerAggregate):
    """Factory function for container-aware registration."""
    surface = McpToolsRegistrySurface(mcp)
    surface.register_tools(container)
