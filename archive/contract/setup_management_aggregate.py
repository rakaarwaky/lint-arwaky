from abc import ABC, abstractmethod
from pydantic import BaseModel, ConfigDict
from .service_container_aggregate import ServiceContainerAggregate
from ..taxonomy import TransportUrlVO, BooleanVO, DirectoryPath, EnvContentVO, McpConfigVO

class SetupManagementAggregate(BaseModel, ABC):
    """AGGREGATE: Domain contract for setup-related orchestration."""
    model_config = ConfigDict(arbitrary_types_allowed=True)
    container: ServiceContainerAggregate | None = None

    @abstractmethod
    def check_http(self, url: TransportUrlVO) -> BooleanVO:
        ...

    @abstractmethod
    def generate_env(self, transport: TransportUrlVO, home: DirectoryPath) -> EnvContentVO:
        ...

    @abstractmethod
    def generate_mcp_config(self, transport: TransportUrlVO) -> McpConfigVO:
        ...

    @abstractmethod
    def mcp_config_claude(self, transport: TransportUrlVO) -> McpConfigVO:
        ...

    @abstractmethod
    def mcp_config_hermes(self, transport: TransportUrlVO) -> McpConfigVO:
        ...

    @abstractmethod
    def mcp_config_vscode(self, transport: TransportUrlVO) -> McpConfigVO:
        ...
