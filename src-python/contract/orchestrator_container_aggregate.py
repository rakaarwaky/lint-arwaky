from abc import ABC, abstractmethod
from pydantic import BaseModel, ConfigDict

class OrchestratorContainerAggregate(BaseModel, ABC):
    """
    AGGREGATE: Defines the orchestrator wiring requirements.
    """
    model_config = ConfigDict(arbitrary_types_allowed=True)

    @abstractmethod
    def _init_orchestrators(self) -> None:
        """ARCHITECTURAL COMMITMENT: Initialize all high-level orchestrators."""
        ...
