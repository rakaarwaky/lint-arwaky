"""pipeline_dispatcher_aggregate - Aggregate contract for pipeline dispatcher."""
from __future__ import annotations
from abc import ABC, abstractmethod
from pydantic import BaseModel, ConfigDict
from ..taxonomy import FilePath, ContentString, MetadataVO, BooleanVO, ResponseData
from .orchestrator_container_aggregate import OrchestratorContainerAggregate

class PipelineActionDispatcherAggregate(BaseModel, ABC):
    """AGGREGATE: Protocol for the pipeline action dispatcher."""
    model_config = ConfigDict(arbitrary_types_allowed=True)
    root_path: FilePath | None = None
    container: OrchestratorContainerAggregate

    @abstractmethod
    async def dispatch(self, action: ContentString, args: MetadataVO) -> ResponseData:
        """Dispatch action to the appropriate use case or tool."""
        pass

    @abstractmethod
    def validate_action(self, action: ContentString) -> BooleanVO:
        """Check if action is known."""
        pass
