from abc import ABC, abstractmethod
from ..taxonomy import ArchitectureConfig, LayerMapVO

class ArchitectureOrchestratorAggregate(ABC):
    """AGGREGATE: Orchestrates the resolution of architectural configurations."""
    @abstractmethod
    def resolve_effective_layer_map(self, config: ArchitectureConfig) -> LayerMapVO:
        """Button: Resolve a config into a flat layer map."""
        ...
