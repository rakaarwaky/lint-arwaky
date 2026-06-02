"""lint_scan_event — Linting domain event types."""

from pydantic import BaseModel, ConfigDict, Field

from .lint_severity_vo import Severity
from .error_code_vo import ErrorCode
from .score_format_vo import Score
from .message_status_vo import Count, ComplianceStatus
from .time_duration_vo import Duration, Timestamp
from .error_value_vo import ErrorMessage
from .file_path_vo import FilePath
from .adapter_name_vo import AdapterName
from .adapter_collection_vo import AdapterNameList


class ScanStarted(BaseModel):
    """Scan began."""

    model_config = ConfigDict(frozen=True)

    path: FilePath
    adapters: AdapterNameList
    timestamp: Timestamp = Field(default_factory=lambda: Timestamp(value=""))


class ScanCompleted(BaseModel):
    """Scan finished."""

    model_config = ConfigDict(frozen=True)

    path: FilePath
    score: Score
    worst_severity: Severity
    violation_count: Count
    duration_ms: Duration
    is_passing: ComplianceStatus = Field(
        default_factory=lambda: ComplianceStatus(value=True)
    )
    timestamp: Timestamp = Field(default_factory=lambda: Timestamp(value=""))


class ScanFailed(BaseModel):
    """Scan failed."""

    model_config = ConfigDict(frozen=True)

    path: FilePath
    adapter: AdapterName
    error_message: ErrorMessage
    error_code: ErrorCode | None = None
    timestamp: Timestamp = Field(default_factory=lambda: Timestamp(value=""))


class FixApplied(BaseModel):
    """Fix applied to a file."""

    model_config = ConfigDict(frozen=True)

    path: FilePath
    adapter: AdapterName
    error_code: ErrorCode
    changes_count: Count
    timestamp: Timestamp = Field(default_factory=lambda: Timestamp(value=""))


class AdapterRegistered(BaseModel):
    """Adapter loaded."""

    model_config = ConfigDict(frozen=True)

    adapter_name: AdapterName
    timestamp: Timestamp = Field(default_factory=lambda: Timestamp(value=""))


class HookInstalled(BaseModel):
    """Pre-commit hook installed."""

    model_config = ConfigDict(frozen=True)

    path: FilePath
    executable: FilePath
    timestamp: Timestamp = Field(default_factory=lambda: Timestamp(value=""))


class HookRemoved(BaseModel):
    """Pre-commit hook removed."""

    model_config = ConfigDict(frozen=True)

    path: FilePath
    timestamp: Timestamp = Field(default_factory=lambda: Timestamp(value=""))
