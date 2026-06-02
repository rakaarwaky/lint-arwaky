"""pipeline_orchestrator_aggregate - Aggregate contract for pipeline orchestrator."""
from abc import ABC, abstractmethod
from pydantic import BaseModel, ConfigDict
from ..taxonomy import FilePath, GovernanceReport
from .linter_adapter_port import ILinterAdapterPort
from .semantic_tracer_port import ISemanticTracerPort
from .service_container_aggregate import ServiceContainerAggregate

class LintPipelineOrchestratorAggregate(BaseModel, ABC):
    """
    AGGREGATE: Defines the required domain context for linting pipeline orchestration.
    """
    model_config = ConfigDict(arbitrary_types_allowed=True)
    root_path: FilePath | None = None
    container: ServiceContainerAggregate

    @property
    @abstractmethod
    def _INTERFACE_PORT(self) -> type[ILinterAdapterPort]:
        """ARCHITECTURAL COMMITMENT: The infrastructure boundary used (adapters)."""
        ...

    @property
    @abstractmethod
    def _INTERFACE_TRACER(self) -> type[ISemanticTracerPort]:
        """ARCHITECTURAL COMMITMENT: The semantic tracing boundary used."""
        ...

    @abstractmethod
    async def run(self, path: FilePath) -> GovernanceReport:
        """Pipeline execution behavior."""
        ...
