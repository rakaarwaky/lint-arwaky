"""Helper functions for CLI setup commands."""

from ..taxonomy import DirectoryPath, EnvContentVO, McpConfigVO
from ..contract import ServiceContainerAggregate


class SetupManagementSurface:
    """Surface for setup management logic."""

    @property
    def _INTERFACE(self) -> object:
        """ARCHITECTURAL COMMITMENT: Required interface."""
        return ServiceContainerAggregate

    container: ServiceContainerAggregate | None = None

    def __init__(self):
        """Initialize surface."""
        pass

    def register_all(self, container: ServiceContainerAggregate) -> None:
        """Setup management registration fulfillment."""
        self.container = container

    def generate_env(self, home: DirectoryPath) -> EnvContentVO:
        """Generate .env content via container."""
        if not self.container:
            raise RuntimeError("SetupManagementSurface container not initialized")

        content = self.container.setup_processor.generate_env(home)
        return content

    def generate_mcp_config(self) -> McpConfigVO:
        """Generate mcp.json entry via container."""
        if not self.container:
            raise RuntimeError("SetupManagementSurface container not initialized")

        config = self.container.setup_processor.generate_mcp_config()
        return config

    def mcp_config_claude(self) -> McpConfigVO:
        """MCP config for Claude Desktop."""
        return self.generate_mcp_config()

    def mcp_config_hermes(self) -> McpConfigVO:
        """MCP config for Hermes Agent."""
        return self.generate_mcp_config()

    def mcp_config_vscode(self) -> McpConfigVO:
        """MCP config for VS Code."""
        base = self.generate_mcp_config().value.value
        return McpConfigVO(value={"mcp": {"servers": base}})


def register_setup_management(container: ServiceContainerAggregate) -> None:
    """Register setup management surface."""
    _get_instance().register_all(container)


# Lazy singleton — created on first call to avoid import-time side effects
_Instance = None


def _get_instance():
    global _Instance
    if _Instance is None:
        _Instance = SetupManagementSurface()
    return _Instance


def generate_env(*args, **kwargs):
    return _get_instance().generate_env(*args, **kwargs)


def generate_mcp_config(*args, **kwargs):
    return _get_instance().generate_mcp_config(*args, **kwargs)


def mcp_config_claude(*args, **kwargs):
    return _get_instance().mcp_config_claude(*args, **kwargs)


def mcp_config_hermes(*args, **kwargs):
    return _get_instance().mcp_config_hermes(*args, **kwargs)


def mcp_config_vscode(*args, **kwargs):
    return _get_instance().mcp_config_vscode(*args, **kwargs)
