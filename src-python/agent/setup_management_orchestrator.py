"""SetupManagementOrchestrator — Implementation of SetupManagementAggregate (Agent Logic)."""

from typing import Type
from ..contract import SetupManagementAggregate, ServiceContainerAggregate
from ..taxonomy import (
    TransportUrlVO,
    BooleanVO,
    DirectoryPath,
    EnvContentVO,
    McpConfigVO,
)


class SetupManagementOrchestrator(SetupManagementAggregate):
    """Orchestrator that handles setup-related domain logic for the agent."""

    container: ServiceContainerAggregate | None = None

    def __init__(self, container: ServiceContainerAggregate) -> None:
        super().__init__(container=container)
        self.container = container

    @property
    def _INTERFACE(self) -> Type[ServiceContainerAggregate]:
        return ServiceContainerAggregate

    def register_all(self, container: ServiceContainerAggregate) -> None:
        pass

    def check_http(self, url: TransportUrlVO) -> BooleanVO:
        if self.container is None:
            raise RuntimeError("Container not initialized")
        # Use capability processor
        is_healthy = self.container.setup_processor.check_http(url.value)
        return BooleanVO(value=is_healthy)

    def generate_env(
        self, transport: TransportUrlVO, home: DirectoryPath
    ) -> EnvContentVO:
        if self.container is None:
            raise RuntimeError("Container not initialized")
        content = self.container.setup_processor.generate_env(
            transport.value, home.value
        )
        return EnvContentVO(value=content)

    def generate_mcp_config(self, transport: TransportUrlVO) -> McpConfigVO:
        if self.container is None:
            raise RuntimeError("Container not initialized")
        config = self.container.setup_processor.generate_mcp_config(transport.value)
        return McpConfigVO(value=config)

    def mcp_config_claude(self, transport: TransportUrlVO) -> McpConfigVO:
        if self.container is None:
            raise RuntimeError("Container not initialized")
        config = self.container.setup_processor.mcp_config_claude(transport.value)
        return McpConfigVO(value=config)

    def mcp_config_hermes(self, transport: TransportUrlVO) -> McpConfigVO:
        if self.container is None:
            raise RuntimeError("Container not initialized")
        config = self.container.setup_processor.mcp_config_hermes(transport.value)
        return McpConfigVO(value=config)

    def mcp_config_vscode(self, transport: TransportUrlVO) -> McpConfigVO:
        if self.container is None:
            raise RuntimeError("Container not initialized")
        config = self.container.setup_processor.mcp_config_vscode(transport.value)
        return McpConfigVO(value=config)
