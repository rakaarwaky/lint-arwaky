"""report_commands_aggregate - Aggregate contract for report commands."""
from abc import ABC, abstractmethod
from pydantic import BaseModel, ConfigDict
from ..taxonomy import FilePath, FileFormat, Identity
from .service_container_aggregate import ServiceContainerAggregate

class ReportCommandsAggregate(BaseModel, ABC):
    """AGGREGATE: Domain contract for report-related surface commands."""
    model_config = ConfigDict(arbitrary_types_allowed=True)
    root_path: FilePath | None = None
    container: ServiceContainerAggregate

    @abstractmethod
    def report(
        self, path: FilePath | Identity, output_format: FileFormat | Identity
    ) -> None:
        """Generate a detailed quality report."""
        ...

    @abstractmethod
    def security(self, path: FilePath | Identity) -> None:
        """Run security-focused scan."""
        ...
