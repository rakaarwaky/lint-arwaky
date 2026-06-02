"""pipeline_input_aggregate - Aggregate contract for pipeline input."""
from abc import ABC, abstractmethod
from pydantic import BaseModel, ConfigDict
from ..taxonomy import FilePath, PatternList, ContentString, MetadataVO, BooleanVO
from .job_registry_aggregate import JobRegistryAggregate
from .arch_compliance_protocol import IArchComplianceProtocol

class PipelineInputAggregate(BaseModel, ABC):
    """AGGREGATE: Input contract for the agent execution pipeline."""
    model_config = ConfigDict(frozen=True, arbitrary_types_allowed=True)
    root_path: FilePath | None = None

    @property
    @abstractmethod
    def _INTERFACE_REGISTRY(self) -> type[JobRegistryAggregate]:
        """ARCHITECTURAL COMMITMENT: Required interface."""
        ...

    @property
    @abstractmethod
    def _INTERFACE_COMPLIANCE(self) -> type[IArchComplianceProtocol]:
        """ARCHITECTURAL COMMITMENT: Required interface."""
        ...

    action: ContentString
    path: FilePath | None = None
    args: MetadataVO | None = None
    rules: PatternList = PatternList(values=[])
    use_retry: BooleanVO | None = None
