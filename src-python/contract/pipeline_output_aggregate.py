"""pipeline_output_aggregate - Aggregate contract for pipeline output."""
from abc import ABC, abstractmethod
from pydantic import BaseModel, ConfigDict
from ..taxonomy import (
    FilePath,
    JobId,
    SuccessStatus,
    ErrorMessage,
    Suggestion,
    ResponseData,
)
from .job_registry_aggregate import JobRegistryAggregate
from .arch_compliance_protocol import IArchComplianceProtocol

class PipelineOutputAggregate(BaseModel, ABC):
    """AGGREGATE: Output contract for the agent pipeline."""
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

    success: SuccessStatus
    job_id: JobId | None = None
    data: ResponseData | None = None
    error: ErrorMessage | None = None
    suggestion: Suggestion | None = None
