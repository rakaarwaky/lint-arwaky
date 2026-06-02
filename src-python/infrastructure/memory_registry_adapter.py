"""Infrastructure adapter for in-memory job tracking."""

from __future__ import annotations

import uuid
import asyncio
import logging

from datetime import datetime, timezone

from ..taxonomy import (
    Count,
    JobId,
    ActionName,
    SuccessStatus,
    ErrorMessage,
    MetadataVO,
    Identity,
    Duration,
    ResponseData,
)
from ..contract import IJobRegistryPort


class MemoryJobRegistryAdapter(IJobRegistryPort):
    """Implementation of job tracking and lifecycle management."""

    def __init__(self) -> None:
        super().__init__()
        self._jobs: dict = {}
        self._lock: asyncio.Lock = asyncio.Lock()
        self.logger = logging.getLogger("infra.jobs")

    def _get_lock(self) -> asyncio.Lock:
        return self._lock

    async def create_job(self, action: ActionName | Identity) -> JobId:
        """Register a new job and return its ID."""
        action_str = str(action.value) if hasattr(action, "value") else str(action)
        job_id_str = str(uuid.uuid4())[:8]
        job_id = JobId(value=job_id_str)
        async with self._get_lock():
            self._jobs[job_id_str] = {
                "id": job_id_str,
                "status": "running",
                "action": action_str,
                "started_at": datetime.now(timezone.utc).isoformat(),
            }
        return job_id

    async def complete_job(
        self, job_id: JobId | Identity, result: ResponseData | MetadataVO
    ):
        """Mark job as completed."""
        jid = str(job_id.value) if hasattr(job_id, "value") else str(job_id)
        async with self._get_lock():
            if jid in self._jobs:
                self._jobs[jid]["status"] = "completed"
                self._jobs[jid]["result"] = result
                self._jobs[jid]["completed_at"] = datetime.now(timezone.utc).isoformat()

    async def fail_job(
        self, job_id: JobId | Identity, error: ErrorMessage | Identity
    ) -> None:
        """Mark job as failed."""
        jid = str(job_id.value) if hasattr(job_id, "value") else str(job_id)
        error_str = str(error.value) if hasattr(error, "value") else str(error)
        async with self._get_lock():
            if jid in self._jobs:
                self._jobs[jid]["status"] = "failed"
                self._jobs[jid]["error"] = error_str
                self._jobs[jid]["failed_at"] = datetime.now(timezone.utc).isoformat()

    async def list_jobs(self) -> MetadataVO:
        """Return all jobs."""
        async with self._get_lock():
            return MetadataVO(data={"jobs": list(self._jobs.values())})

    async def get_job(self, job_id: JobId | Identity) -> MetadataVO | None:
        """Return a single job or None."""
        jid = str(job_id.value) if hasattr(job_id, "value") else str(job_id)
        async with self._get_lock():
            job = self._jobs.get(jid)
            if not job:
                return None
            return MetadataVO(data=job)

    async def cancel_job(self, job_id: JobId | Identity) -> SuccessStatus:
        """Cancel a running job. Returns SuccessStatus(True) if cancelled."""
        jid = str(job_id.value) if hasattr(job_id, "value") else str(job_id)
        async with self._get_lock():
            if jid in self._jobs:
                self._jobs[jid]["status"] = "cancelled"
                self._jobs[jid]["completed_at"] = datetime.now(timezone.utc).isoformat()
                return SuccessStatus(status=True)
            return SuccessStatus(status=False)

    async def run_with_retry(
        self,
        operation: Identity,
        max_retries: Count = Count(value=5),
        base_delay: Duration = Duration(value=0.5),
    ) -> ResponseData:
        """Execute an operation with exponential backoff retry logic.

        Note: This is a mock implementation.
        """
        return ResponseData(content=f"Executed {operation}")
