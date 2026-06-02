"""watch_service_error — Watch service domain error types."""

from pydantic import BaseModel, ConfigDict

from .error_code_vo import ErrorCode
from .error_value_vo import (
    ErrorMessage,
    Cause,
)
from .file_path_vo import FilePath


class WatchServiceError(BaseModel):
    """General failure during file system watching."""

    model_config = ConfigDict(frozen=True)

    path: FilePath | None = None
    message: ErrorMessage
    error_code: ErrorCode | None = None
    cause: Cause | None = None

    def __str__(self):
        target = f" on {self.path}" if self.path else ""
        code = f" [{self.error_code}]" if self.error_code else ""
        return f"Watch Error{target}{code}: {self.message}"


class WatchSubscriptionError(WatchServiceError):
    """Failed to subscribe to path changes (e.g., too many watchers)."""
    pass


class WatchEventError(WatchServiceError):
    """Failed to process a specific file event."""
    pass
