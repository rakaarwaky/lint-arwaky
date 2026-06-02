"""file_system_error — File system domain error types."""

from pydantic import BaseModel, ConfigDict

from .error_code_vo import ErrorCode
from .error_value_vo import (
    ErrorMessage,
    Cause,
)
from .file_path_vo import FilePath
from .job_action_vo import ActionName


class FileSystemError(BaseModel):
    """General file system operation failure."""

    model_config = ConfigDict(frozen=True)

    path: FilePath
    message: ErrorMessage
    operation: ActionName  # e.g., 'read', 'write', 'delete', 'walk'
    error_code: ErrorCode | None = None
    cause: Cause | None = None

    def __str__(self):
        code = f" [{self.error_code}]" if self.error_code else ""
        return f"FS Error during {self.operation} on {self.path}{code}: {self.message}"


class PathNotFoundError(FileSystemError):
    """Target path does not exist on disk."""

    def __str__(self):
        return f"Path not found: {self.path} ({self.message})"


class AccessDeniedError(FileSystemError):
    """Permission denied for the requested operation."""

    def __str__(self):
        return f"Access denied: {self.path} ({self.message})"
