"""lint_adapter_error — Linting domain error types."""

from pydantic import BaseModel, ConfigDict, Field

from .error_code_vo import ErrorCode
from .error_value_vo import (
    ErrorMessage,
    ExitCode,
    FieldName,
    Constraint,
    ActualValue,
    Cause,
)
from .file_path_vo import FilePath
from .adapter_name_vo import AdapterName
from .lint_domain_vo import CommandArgs


class AdapterError(BaseModel):
    """External linter tool failed."""

    model_config = ConfigDict(frozen=True)

    adapter_name: AdapterName
    message: ErrorMessage
    error_code: ErrorCode | None = None
    command: CommandArgs = CommandArgs()
    stderr: ErrorMessage = Field(default_factory=lambda: ErrorMessage(value=""))
    exit_code: ExitCode | None = None

    def __str__(self):
        code = f" [{self.error_code}]" if self.error_code else ""
        return f"[{self.adapter_name}]{code} {self.message}"


class ScanError(BaseModel):
    """Scan operation failed on a path."""

    model_config = ConfigDict(frozen=True)

    path: FilePath
    message: ErrorMessage
    error_code: ErrorCode | None = None
    adapter_name: AdapterName | None = None
    cause: Cause | None = None

    def __str__(self):
        adapter = f" ({self.adapter_name})" if self.adapter_name else ""
        code = f" [{self.error_code}]" if self.error_code else ""
        return f"Scan failed{adapter}{code}: {self.path} — {self.message}"


class ValidationError(BaseModel):
    """Input validation failed."""

    model_config = ConfigDict(frozen=True)

    field_name: FieldName
    message: ErrorMessage
    constraint: Constraint = Field(default_factory=lambda: Constraint(value=""))
    value: ActualValue = Field(default_factory=lambda: ActualValue(value=""))

    def __str__(self):
        return f"Validation failed on '{self.field_name}': {self.message}"
