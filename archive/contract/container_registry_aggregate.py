from abc import ABC, abstractmethod
from ..taxonomy import DirectoryPath
from .service_container_aggregate import ServiceContainerAggregate

class ContainerRegistryAggregate(ABC):
    """
    AGGREGATE: Registry for the Agent DI containers.
    """
    @staticmethod
    @abstractmethod
    def get_container(project_root: DirectoryPath | None = None) -> ServiceContainerAggregate:
        """Get or create a container for a specific project root."""
        ...

    @staticmethod
    @abstractmethod
    def reset_container(project_root: DirectoryPath | None = None) -> None:
        """Reset container(s) in the registry."""
        ...
