"""setup_management_protocol — Protocol interface for setup and configuration management."""

from abc import ABC, abstractmethod
from ..taxonomy import (
    DirectoryPath,
    EnvContentVO,
    McpConfigVO,
)


class ISetupManagementProtocol(ABC):
    """Protocol for setup and configuration management."""

    @abstractmethod
    def generate_env(self, home: DirectoryPath) -> EnvContentVO:
        """Generate .env content."""
        ...

    @abstractmethod
    def generate_mcp_config(self) -> McpConfigVO:
        """Generate mcp.json entry."""
        ...

    @abstractmethod
    def mcp_config_claude(self) -> McpConfigVO:
        """Claude Desktop MCP config."""
        ...

    @abstractmethod
    def mcp_config_hermes(self) -> McpConfigVO:
        """Hermes MCP config."""
        ...

    @abstractmethod
    def mcp_config_vscode(self) -> McpConfigVO:
        """VS Code MCP config."""
        ...
