"""job_registry_error — Job management domain error types."""

from pydantic import BaseModel, ConfigDict

from .error_code_vo import ErrorCode
from .error_value_vo import (
    ErrorMessage,
    Cause,
)
from .job_action_vo import JobId


class JobError(BaseModel):
    """General failure during job management or registry operations."""

    model_config = ConfigDict(frozen=True)

    job_id: JobId | None = None
    message: ErrorMessage
    error_code: ErrorCode | None = None
    cause: Cause | None = None

    def __str__(self):
        target = f" for job {self.job_id}" if self.job_id else ""
        code = f" [{self.error_code}]" if self.error_code else ""
        return f"Job Error{target}{code}: {self.message}"
