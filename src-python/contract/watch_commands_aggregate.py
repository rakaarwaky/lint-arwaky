"""watch_commands_aggregate - Aggregate contract for watch commands."""
from abc import ABC, abstractmethod
from pydantic import BaseModel, ConfigDict
from ..taxonomy import FilePath, Identity
from .service_container_aggregate import ServiceContainerAggregate

class WatchCommandsAggregate(BaseModel, ABC):
    """AGGREGATE: Domain contract for watch-related surface commands."""
    model_config = ConfigDict(arbitrary_types_allowed=True)
    root_path: FilePath | None = None
    container: ServiceContainerAggregate

    @abstractmethod
    def watch(self, path: FilePath | Identity) -> None:
        """Watch for file changes and run linters automatically."""
        ...
