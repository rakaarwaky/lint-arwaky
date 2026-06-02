"""analysis_orchestrator_aggregate - Aggregate contract for analysis orchestrator."""
from abc import ABC, abstractmethod
from pydantic import BaseModel, ConfigDict
from ..taxonomy import FilePath, GovernanceReport
from .service_container_aggregate import ServiceContainerAggregate

class AnalysisOrchestratorAggregate(BaseModel, ABC):
    """AGGREGATE: Domain contract for analysis orchestration."""
    model_config = ConfigDict(arbitrary_types_allowed=True)

    @property
    @abstractmethod
    def _INTERFACE_CONTAINER(self) -> type[ServiceContainerAggregate]:
        """ARCHITECTURAL COMMITMENT: The orchestration boundary (Composition Root)."""
        ...

    container: ServiceContainerAggregate

    @abstractmethod
    async def get_complexity(self, path: FilePath) -> GovernanceReport:
        """Execute complexity analysis."""

    @abstractmethod
    async def get_duplicates(self, path: FilePath) -> GovernanceReport:
        """Execute duplication analysis."""

    @abstractmethod
    async def get_trends(self, path: FilePath) -> GovernanceReport:
        """Execute quality trends analysis."""

    @abstractmethod
    async def get_dependencies(self, path: FilePath) -> GovernanceReport:
        """Execute dependency vulnerability analysis."""

    @abstractmethod
    async def run(self, path: FilePath) -> GovernanceReport:
        """Execute full project analysis."""
