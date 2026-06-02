from abc import ABC, abstractmethod
from typing import Type, TypeVar
from ..taxonomy import FilePath

T = TypeVar("T")

class ServiceContainerAggregate(ABC):
    """
    AGGREGATE: Main service container contract.
    Flat hierarchy: inherits ONLY from ABC.
    """
    @abstractmethod
    def get_aggregate(self, io_contract: Type[T]) -> T | None:
        """Resolve high-level Aggregate contracts."""
        ...

    @abstractmethod
    def get(self, interface: Type[T]) -> T | None:
        """Internal service discovery."""
        ...

    @abstractmethod
    def get_for_path(self, path: FilePath) -> "ServiceContainerAggregate":
        """Get or create a container instance for a specific path."""
        ...
