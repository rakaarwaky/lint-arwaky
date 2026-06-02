"""watch_orchestrator_aggregate - Aggregate contract for watch orchestrator."""
from abc import ABC, abstractmethod
from pydantic import BaseModel, ConfigDict
from ..taxonomy import FilePath
from .job_registry_aggregate import JobRegistryAggregate
from .service_container_aggregate import ServiceContainerAggregate

class WatchExecutionOrchestratorAggregate(BaseModel, ABC):
    """
    AGGREGATE: Defines the watch orchestration boundaries.
    """
    model_config = ConfigDict(arbitrary_types_allowed=True)
    root_path: FilePath | None = None

    @property
    @abstractmethod
    def _INTERFACE_JOB_REGISTRY(self) -> type[JobRegistryAggregate]:
        """ARCHITECTURAL COMMITMENT: Required infrastructure for job tracking."""
        ...

    container: ServiceContainerAggregate
