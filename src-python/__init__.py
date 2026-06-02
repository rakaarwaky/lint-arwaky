"""Auto Linter - Autonomous multi-language linting and architecture compliance auditing."""

from .cli_main_entry import main as cli_main
from .mcp_main_entry import main as mcp_main

__all__ = ["cli_main", "mcp_main"]
