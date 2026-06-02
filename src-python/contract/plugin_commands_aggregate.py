"""plugin_commands_aggregate - Aggregate contract for plugin commands."""
from abc import ABC, abstractmethod
from pydantic import BaseModel, ConfigDict
from ..taxonomy import FilePath
from .service_container_aggregate import ServiceContainerAggregate

class PluginCommandsAggregate(BaseModel, ABC):
    """AGGREGATE: Domain contract for plugin-related surface commands."""
    model_config = ConfigDict(arbitrary_types_allowed=True)
    root_path: FilePath | None = None
    container: ServiceContainerAggregate

    @abstractmethod
    def adapters(self) -> None:
        """List enabled adapters."""
        ...

    @abstractmethod
    def plugins(self) -> None:
        """List discovered and registered plugins."""
        ...
