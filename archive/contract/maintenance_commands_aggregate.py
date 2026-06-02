"""maintenance_commands_aggregate - Aggregate contract for maintenance commands."""
from abc import ABC, abstractmethod
from ..taxonomy import JobId, FilePath, MaintenanceStatsVO, DoctorResultVO
from pydantic import BaseModel, ConfigDict
from .service_container_aggregate import ServiceContainerAggregate

class MaintenanceCommandsAggregate(BaseModel, ABC):
    """AGGREGATE: Defines the interface for maintenance commands."""
    model_config = ConfigDict(arbitrary_types_allowed=True)
    container: ServiceContainerAggregate

    @abstractmethod
    def stats(self, project_path: FilePath) -> MaintenanceStatsVO:
        """Show statistics dashboard."""
        ...

    @abstractmethod
    def clean(self) -> None:
        """Cleanup cache and temporary files."""
        ...

    @abstractmethod
    def update(self) -> None:
        """Update linter adapters."""
        ...

    @abstractmethod
    def doctor(self) -> DoctorResultVO:
        """Diagnose common issues."""
        ...

    @abstractmethod
    def cancel(self, job_id: JobId) -> None:
        """Cancel a running job."""
        ...
