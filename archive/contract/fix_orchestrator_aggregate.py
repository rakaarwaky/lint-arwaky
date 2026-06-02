"""fix_orchestrator_aggregate - Aggregate contract for fix orchestrator."""
from abc import ABC, abstractmethod
from pydantic import BaseModel, ConfigDict
from ..taxonomy import FilePath, FixResult
from .linter_adapter_port import ILinterAdapterPort
from .file_system_port import IFileSystemPort
from .service_container_aggregate import ServiceContainerAggregate

class LintFixOrchestratorAggregate(BaseModel, ABC):
    """
    AGGREGATE: Defines the required domain context for automatic fix orchestration.
    """
    model_config = ConfigDict(arbitrary_types_allowed=True)
    container: ServiceContainerAggregate

    @property
    @abstractmethod
    def _INTERFACE_PORT(self) -> type[ILinterAdapterPort]:
        """ARCHITECTURAL COMMITMENT: The infrastructure boundary used (adapters)."""
        ...

    @property
    @abstractmethod
    def _INTERFACE_FS(self) -> type[IFileSystemPort]:
        """ARCHITECTURAL COMMITMENT: The file system boundary used."""
        ...

    @abstractmethod
    def execute(self, path: FilePath) -> FixResult:
        """Fix execution behavior."""
        ...
