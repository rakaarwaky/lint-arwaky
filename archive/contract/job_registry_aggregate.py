"""job_registry_aggregate - Aggregate contract for job registry."""
from abc import ABC, abstractmethod
from .job_registry_port import IJobRegistryPort
from ..taxonomy import (
    JobId,
    ActionName,
    Identity,
    MetadataVO,
    SuccessStatus,
    ErrorMessage,
    Count,
    Duration,
    ResponseData,
)

class JobRegistryAggregate(ABC):
    """
    AGGREGATE: Architectural aggregator for Job Registry capabilities.
    """
    @property
    @abstractmethod
    def _INTERFACE_PORT(self) -> type[IJobRegistryPort]:
        """ARCHITECTURAL COMMITMENT: Required port."""
        ...

    @abstractmethod
    async def create_job(self, action: ActionName | Identity) -> JobId:
        """Register a new job."""
        ...

    @abstractmethod
    async def complete_job(
        self, job_id: JobId | Identity, result: ResponseData | MetadataVO
    ):
        """Finalize job success."""
        ...

    @abstractmethod
    async def fail_job(self, job_id: JobId | Identity, error: ErrorMessage | Identity):
        """Finalize job failure."""
        ...

    @abstractmethod
    async def list_jobs(self) -> MetadataVO:
        """Return all jobs."""
        ...

    @abstractmethod
    async def get_job(self, job_id: JobId | Identity) -> MetadataVO | None:
        """Return a single job or None."""
        ...

    @abstractmethod
    async def cancel_job(self, job_id: JobId | Identity) -> SuccessStatus:
        """Cancel a running job."""
        ...

    @abstractmethod
    async def run_with_retry(
        self,
        operation: Identity,
        max_retries: Count = Count(value=5),
        base_delay: Duration = Duration(value=0.5),
    ) -> ResponseData:
        """Execute with retry."""
        ...
