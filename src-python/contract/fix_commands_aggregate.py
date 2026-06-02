"""fix_commands_aggregate - Aggregate contract for fix commands."""
from abc import ABC, abstractmethod
from ..taxonomy import FilePath
from pydantic import BaseModel, ConfigDict
from .service_container_aggregate import ServiceContainerAggregate

class FixCommandsAggregate(BaseModel, ABC):
    """AGGREGATE: Interface for lint fixing logic."""
    model_config = ConfigDict(arbitrary_types_allowed=True)
    container: ServiceContainerAggregate

    @abstractmethod
    def fix(self, project_path: FilePath) -> None:
        """Apply safe fixes automatically (Ruff, ESLint, Prettier)."""
        pass
