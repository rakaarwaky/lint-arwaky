"""project_container_aggregate - Aggregate contract for project container."""
from __future__ import annotations
from abc import ABC, abstractmethod
from typing import TypeVar
from ..taxonomy import FilePath, GovernanceReport, MetadataVO

T = TypeVar("T")

class ProjectContainerAggregate(ABC):
    """
    AGGREGATE: High-level contract for the Project Container.
    """
    root_path: FilePath | None = None

    @abstractmethod
    async def run_analysis(self, path: FilePath) -> GovernanceReport:
        """Button: Trigger a full analysis for a given path."""
        ...

    @abstractmethod
    async def get_health(self) -> MetadataVO:
        """Button: Orchestrate and return system health data."""
        ...

    @abstractmethod
    def get_aggregate(self, io_contract: type[T]) -> T:
        """Button: Resolve a specific aggregate contract."""
        ...
