from abc import ABC, abstractmethod
from pydantic import BaseModel, ConfigDict

class AdapterContainerAggregate(BaseModel, ABC):
    """
    AGGREGATE: Defines the adapter wiring requirements.
    """
    model_config = ConfigDict(arbitrary_types_allowed=True)

    @abstractmethod
    def _init_adapters(self) -> None:
        """ARCHITECTURAL COMMITMENT: Initialize all linter adapters."""
        ...
