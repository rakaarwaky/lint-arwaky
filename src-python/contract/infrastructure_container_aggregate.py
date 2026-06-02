from abc import ABC, abstractmethod
from pydantic import BaseModel, ConfigDict
from ..taxonomy import FilePath

class InfrastructureContainerAggregate(BaseModel, ABC):
    """
    AGGREGATE: Defines the infrastructure wiring requirements.
    """
    model_config = ConfigDict(arbitrary_types_allowed=True)
    root_path: FilePath | None = None

    @abstractmethod
    def _init_infrastructure(self) -> None:
        """ARCHITECTURAL COMMITMENT: Initialize all infrastructure adapters."""
        ...
