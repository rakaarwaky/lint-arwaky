"""execution_orchestrator_aggregate - Aggregate contract for execution orchestrator."""
from abc import ABC, abstractmethod
from typing import Type
from pydantic import BaseModel, ConfigDict
from .job_registry_aggregate import JobRegistryAggregate
from .service_container_aggregate import ServiceContainerAggregate

class PipelineExecutionOrchestratorAggregate(BaseModel, ABC):
    """AGGREGATE: Defines the brain stem's architectural commitments."""
    model_config = ConfigDict(arbitrary_types_allowed=True)

    @property
    @abstractmethod
    def _INTERFACE_JOB_REGISTRY(self) -> Type[JobRegistryAggregate]:
        """ARCHITECTURAL COMMITMENT: Required infrastructure for job tracking."""
        ...

    container: ServiceContainerAggregate | None = None
