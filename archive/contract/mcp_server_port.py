"""mcp_server_port — Port interface for MCP server lifecycle and tool management."""

from abc import ABC, abstractmethod
from typing import Any, Callable, Coroutine

from ..taxonomy import SymbolName, DescriptionVO


class IMcpServerPort(ABC):
    """Port for managing an MCP server instance."""

    @abstractmethod
    def register_tool(
        self,
        name: SymbolName,
        description: DescriptionVO,
        handler: Callable[..., Coroutine[Any, Any, Any]],
    ) -> None:
        """Register a tool with the MCP server."""
        ...

    @abstractmethod
    async def run_server(self) -> None:
        """Start the MCP server."""
        ...

    @abstractmethod
    def stop_server(self) -> None:
        """Stop the MCP server."""
        ...
