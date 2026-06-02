"""Implementation of JobRegistryAggregate (Agent Layer)."""

from __future__ import annotations

from ..contract import JobRegistryAggregate, IJobRegistryPort
from ..taxonomy import (
    JobId,
    ActionName,
    Identity,
    MetadataVO,
    SuccessStatus,
    ErrorMessage,
    ResponseData,
    Count,
    Duration,
)


class JobRegistry(JobRegistryAggregate):
    """Agent-level implementation of JobRegistryAggregate.

    This class grounds the JobRegistryAggregate contract by delegating to the
    IJobRegistryPort provided by the infrastructure layer.
    """

    def __init__(self, port: IJobRegistryPort) -> None:
        self._port = port

    @property
    def _INTERFACE_PORT(self):
        return IJobRegistryPort

    async def create_job(self, action: ActionName | Identity) -> JobId:
        return await self._port.create_job(action)

    async def complete_job(
        self, job_id: JobId | Identity, result: ResponseData | MetadataVO
    ):
        await self._port.complete_job(job_id, result)

    async def fail_job(self, job_id: JobId | Identity, error: ErrorMessage | Identity):
        await self._port.fail_job(job_id, error)

    async def list_jobs(self) -> MetadataVO:
        return await self._port.list_jobs()

    async def get_job(self, job_id: JobId | Identity) -> MetadataVO | None:
        return await self._port.get_job(job_id)

    async def cancel_job(self, job_id: JobId | Identity) -> SuccessStatus:
        return await self._port.cancel_job(job_id)

    async def run_with_retry(
        self,
        operation: Identity,
        max_retries: Count = Count(value=5),
        base_delay: Duration = Duration(value=0.5),
    ) -> ResponseData:
        return await self._port.run_with_retry(operation, max_retries, base_delay)
