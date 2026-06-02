"""pipeline_extended_aggregate - Aggregate contract for pipeline extended."""
from __future__ import annotations
from abc import ABC, abstractmethod
from pydantic import BaseModel, ConfigDict
from ..taxonomy import FilePath
from .orchestrator_container_aggregate import OrchestratorContainerAggregate
from .directory_watch_aggregate import DirectoryWatchAggregate
from .multi_project_aggregate import MultiProjectAggregate
from .pipeline_output_aggregate import PipelineOutputAggregate

class PipelineExtendedOrchestratorAggregate(BaseModel, ABC):
    """AGGREGATE: Protocol for the extended pipeline orchestrator."""
    model_config = ConfigDict(arbitrary_types_allowed=True)
    root_path: FilePath | None = None
    container: OrchestratorContainerAggregate

    @abstractmethod
    def execute_multi_project(
        self,
        request: MultiProjectAggregate,
        use_retry: bool | None = None,
        config_path: FilePath | None = None,
    ) -> PipelineOutputAggregate:
        """Orchestrate linting across multiple projects."""
        ...

    @abstractmethod
    def execute_watch(self, request: DirectoryWatchAggregate) -> PipelineOutputAggregate:
        """Orchestrate watching a directory for changes."""
        ...
