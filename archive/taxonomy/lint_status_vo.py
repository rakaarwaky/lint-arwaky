"""lint_status_vo — Status and messaging value objects."""

from enum import Enum
from pydantic import BaseModel, ConfigDict, model_validator, Field
from .log_suggestion_vo import (
    BooleanVO,
    StdOutput,
    StdError,
    MetadataVO,
    ClassPath,
    DescriptionVO,
)
from .error_value_vo import ExitCode
from .adapter_name_vo import AdapterName


class JobStatus(str, Enum):
    """Execution status of a job."""

    PENDING = "pending"
    RUNNING = "running"
    COMPLETED = "completed"
    FAILED = "failed"


class SuccessStatus(BaseModel):
    """Boolean wrapper for execution status."""

    model_config = ConfigDict(frozen=True)
    value: BooleanVO

    @model_validator(mode="before")
    @classmethod
    def coerce_from_primitive(cls, data: object) -> object:
        if isinstance(data, (bool, BooleanVO)):
            return {"value": data}
        return data

    def __bool__(self) -> bool:
        return bool(self.value)

    def __str__(self) -> str:
        return "SUCCESS" if self.value else "FAILURE"


class ActionArgs(BaseModel):
    """Dictionary wrapper for action arguments."""

    model_config = ConfigDict(frozen=True)
    value: MetadataVO = Field(default_factory=MetadataVO)

    def get(self, key: str, default: object = None) -> object:
        return self.value.get(key, default)


class ResponseData(BaseModel):
    """Structured response from command execution."""

    model_config = ConfigDict(frozen=True)
    value: object = None
    stdout: StdOutput = Field(default_factory=lambda: StdOutput(value=""))
    stderr: StdError = Field(default_factory=lambda: StdError(value=""))
    returncode: ExitCode = Field(default_factory=lambda: ExitCode(value=0))
    metadata: MetadataVO = Field(default_factory=MetadataVO)

    def get(self, key: str, default: object = None) -> object:
        """Proxy dictionary access to the value field."""
        if isinstance(self.value, dict):
            return self.value.get(key, default)
        return getattr(self.value, key, default) if self.value else default

    def __getitem__(self, key: str) -> object:
        """Allow subscript access."""
        if isinstance(self.value, dict):
            return self.value[key]
        return getattr(self.value, key)

    @model_validator(mode="before")
    @classmethod
    def sync_value_and_fields(cls, data: object) -> object:
        if isinstance(data, dict):
            cls._coerce_primitives(data)
            val = data.get("value")
            if val and isinstance(val, dict):
                cls._sync_field_value(data, val)
            elif not val:
                cls._sync_other_fields(data)
        return data

    @classmethod
    def _coerce_primitives(cls, data: dict) -> None:
        """Coerce primitive field values to their VO wrappers in-place."""
        if "stdout" in data and isinstance(data["stdout"], str):
            from .log_suggestion_vo import StdOutput

            data["stdout"] = StdOutput(value=data["stdout"])
        if "stderr" in data and isinstance(data["stderr"], str):
            from .log_suggestion_vo import StdError

            data["stderr"] = StdError(value=data["stderr"])
        if "returncode" in data and isinstance(data["returncode"], int):
            from .error_value_vo import ExitCode

            data["returncode"] = ExitCode(value=data["returncode"])
        if "metadata" in data and isinstance(data["metadata"], dict):
            from .log_suggestion_vo import MetadataVO

            data["metadata"] = MetadataVO(value=data["metadata"])

    @classmethod
    def _sync_field_value(cls, data: dict, val: dict) -> None:
        """Copy fields from the inner value dict to top-level data when missing."""
        for field in ["stdout", "stderr", "returncode", "metadata"]:
            if not data.get(field) and field in val:
                data[field] = val[field]

    @classmethod
    def _sync_other_fields(cls, data: dict) -> None:
        """Populate value dict from top-level fields when value is empty."""
        data["value"] = {
            "stdout": data.get("stdout", ""),
            "stderr": data.get("stderr", ""),
            "returncode": data.get("returncode", 0),
            "metadata": data.get("metadata", {}),
        }


class AdapterMetadata(BaseModel):
    """Metadata for a discovered adapter."""

    model_config = ConfigDict(frozen=True)
    name: AdapterName
    class_path: ClassPath
    description: DescriptionVO = Field(default_factory=lambda: DescriptionVO(value=""))


class EnvContentVO(BaseModel):
    """Generated .env file content."""

    model_config = ConfigDict(frozen=True)
    value: str

    @model_validator(mode="before")
    @classmethod
    def coerce_from_primitive(cls, data: object) -> object:
        if isinstance(data, str):
            return {"value": data}
        return data

    def __str__(self) -> str:
        return self.value


class TransportUrlVO(BaseModel):
    """Transport endpoint URL or socket path."""

    model_config = ConfigDict(frozen=True)
    value: str

    @model_validator(mode="before")
    @classmethod
    def coerce_from_primitive(cls, data: object) -> object:
        if isinstance(data, str):
            return {"value": data}
        return data

    def __str__(self) -> str:
        return self.value


class McpConfigVO(BaseModel):
    """MCP configuration dictionary wrapper."""

    model_config = ConfigDict(frozen=True)
    value: MetadataVO


ResponseData.model_rebuild()
