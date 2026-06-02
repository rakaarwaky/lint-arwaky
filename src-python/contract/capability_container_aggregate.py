from abc import ABC, abstractmethod
from pydantic import BaseModel, ConfigDict

class CapabilityContainerAggregate(BaseModel, ABC):
    """
    AGGREGATE: Defines the capability/analyzer wiring requirements.
    """
    model_config = ConfigDict(arbitrary_types_allowed=True)

    @abstractmethod
    def _init_capabilities(self) -> None:
        """ARCHITECTURAL COMMITMENT: Initialize all domain capabilities."""
        ...
