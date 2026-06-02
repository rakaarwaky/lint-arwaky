"""Main entry point for the Auto Linter MCP server.

This module exposes a FastMCP server that registers all tool
operations defined in the AES architecture.
"""

import os
from dotenv import load_dotenv
from ..contract import ServiceContainerAggregate
from mcp.server.fastmcp import FastMCP
from .mcp_tools_store import register_tools

load_dotenv()


class McpServerHandlerSurface:
    """Main surface for the Auto Linter MCP server."""

    def __init__(self) -> None:
        self.server_name = "auto-linter"
        self.mcp = FastMCP(self.server_name)

    def _setup_environment(self) -> None:
        """Ensure virtual environment bin is in the system PATH."""
        import sys

        venv_bin = os.path.dirname(sys.executable)
        current_path = os.environ.get("PATH", "")
        if venv_bin not in current_path:
            os.environ["PATH"] = venv_bin + os.path.pathsep + current_path

    def run_server(self, container: ServiceContainerAggregate) -> None:
        """Run the MCP server fulfillment."""
        self._setup_environment()
        register_tools(self.mcp, container)
        self.mcp.run()

    def run(self, container: ServiceContainerAggregate) -> None:
        """Alias for run_server for compatibility."""
        self.run_server(container)
