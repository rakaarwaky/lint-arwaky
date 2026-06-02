"""metrics_provider_error — Technical metrics domain error types."""

from pydantic import BaseModel, ConfigDict

from .error_code_vo import ErrorCode
from .error_value_vo import (
    ErrorMessage,
    Cause,
)
from .file_path_vo import FilePath


class MetricsError(BaseModel):
    """General failure during metrics retrieval."""

    model_config = ConfigDict(frozen=True)

    path: FilePath | None = None
    message: ErrorMessage
    error_code: ErrorCode | None = None
    cause: Cause | None = None

    def __str__(self):
        target = f" for {self.path}" if self.path else ""
        code = f" [{self.error_code}]" if self.error_code else ""
        return f"Metrics Error{target}{code}: {self.message}"
