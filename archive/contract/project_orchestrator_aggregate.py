"""project_orchestrator_aggregate - Aggregate contract for project orchestrator."""
from abc import ABC, abstractmethod
from pydantic import BaseModel, ConfigDict
from ..taxonomy import (
    FilePath,
    ProjectResult,
    AggregatedResults,
    Count,
    FilePathList,
    Identity,
)
from .service_container_aggregate import ServiceContainerAggregate

class MultiProjectOrchestratorAggregate(BaseModel, ABC):
    """
    AGGREGATE: Defines the required domain context for multi-project orchestration.
    """
    model_config = ConfigDict(arbitrary_types_allowed=True)
    root_path: FilePath | None = None
    container: ServiceContainerAggregate

    @property
    @abstractmethod
    def _INTERFACE_FS(self) -> Identity:
        """ARCHITECTURAL COMMITMENT: The infrastructure boundary for file operations."""
        ...

    @property
    @abstractmethod
    def _INTERFACE_CONFIG(self) -> Identity:
        """ARCHITECTURAL COMMITMENT: The infrastructure boundary for config discovery."""
        ...

    @abstractmethod
    async def analyze_project(self, path: FilePath) -> ProjectResult:
        """Single project analysis behavior."""
        ...

    @abstractmethod
    async def scan_all_projects(
        self,
        paths: FilePathList,
        max_concurrency: Count = Count(value=10),
    ) -> AggregatedResults:
        """Batch project scanning behavior."""
        ...

    @staticmethod
    @abstractmethod
    def load_config(config_path: FilePath | Identity) -> FilePathList:
        """Load list of project paths from a config file."""
        ...

    @staticmethod
    @abstractmethod
    def find_projects(
        root: FilePath, config_name: Identity = Identity(value=".auto_linter.json")
    ) -> FilePathList:
        """Find all projects with auto-linter configs."""
        ...
