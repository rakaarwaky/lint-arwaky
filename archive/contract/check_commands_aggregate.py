"""check_commands_aggregate - Aggregate contract for check commands."""
from abc import ABC, abstractmethod
from ..taxonomy import FilePath, ComplianceStatus
from pydantic import BaseModel, ConfigDict
from .service_container_aggregate import ServiceContainerAggregate

class CheckCommandsAggregate(BaseModel, ABC):
    """AGGREGATE: Interface for check-related CLI commands."""
    model_config = ConfigDict(arbitrary_types_allowed=True)
    container: ServiceContainerAggregate

    @abstractmethod
    def check(self, path: FilePath, git_diff: ComplianceStatus) -> None:
        """Run all linters and check architecture compliance score."""
        pass

    @abstractmethod
    def scan(self, path: FilePath) -> None:
        """Full deep scan of a directory (alias for check)."""
        pass
