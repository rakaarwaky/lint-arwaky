from __future__ import annotations
from abc import ABC, abstractmethod


from ..taxonomy import (
    JobId,
    ActionName,
    MetadataVO,
    SuccessStatus,
    Count,
    Duration,
    ErrorMessage,
    Identity,
    ResponseData,
    JobError,
)


class IJobRegistryPort(ABC):
    """Port for job tracking and lifecycle management."""

    @abstractmethod
    async def create_job(self, action: ActionName | Identity) -> JobId | JobError:
        """Register a new job and return its ID."""
        ...

    @abstractmethod
    async def complete_job(
        self, job_id: JobId | Identity, result: ResponseData | MetadataVO
    ):
        """Mark job as completed."""
        ...

    @abstractmethod
    async def fail_job(self, job_id: JobId | Identity, error: ErrorMessage | Identity):
        """Mark job as failed."""
        ...

    @abstractmethod
    async def list_jobs(self) -> MetadataVO:
        """Return all jobs."""
        ...

    @abstractmethod
    async def get_job(self, job_id: JobId | Identity) -> MetadataVO | JobError | None:
        """Return a single job or None."""
        ...

    @abstractmethod
    async def cancel_job(self, job_id: JobId | Identity) -> SuccessStatus | JobError:
        """Cancel a running job. Returns SuccessStatus(True) if cancelled."""
        ...

    @abstractmethod
    async def run_with_retry(
        self,
        operation: Identity,
        max_retries: Count = Count(value=5),
        base_delay: Duration = Duration(value=0.5),
    ) -> ResponseData:
        """Execute async function with exponential backoff retry."""
        ...
