"""log_suggestion_vo — Log output and suggestion value objects."""

from typing import Any
from pydantic import BaseModel, ConfigDict, model_validator, Field


class LogOutput(BaseModel):
    """Execution log output wrapper."""

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

    def __contains__(self, item: str) -> bool:
        return item in self.value

    def __getattr__(self, name: str):
        """Delegate string methods to self.value for backward compatibility."""
        if name in {
            "__reduce_ex__",
            "__reduce__",
            "__getnewargs__",
            "__getstate__",
            "__setstate__",
            "__copy__",
            "__deepcopy__",
            "__class__",
        }:
            raise AttributeError(
                f"'{self.__class__.__name__}' has no attribute '{name}'"
            )
        if hasattr(self.value, name):
            attr = getattr(self.value, name)
            if callable(attr):

                def wrapper(*args, **kwargs):
                    return attr(*args, **kwargs)

                return wrapper
            return attr
        raise AttributeError(
            f"'{self.__class__.__name__}' and its value have no attribute '{name}'"
        )

    def __eq__(self, other: object) -> bool:
        if isinstance(other, LogOutput):
            return self.value == other.value
        if isinstance(other, str):
            return self.value == other
        return NotImplemented


class StdOutput(LogOutput):
    """Standard output wrapper with stdout-specific behavior."""

    def is_empty(self) -> bool:
        """Check if stdout is empty."""
        return not self.value or not self.value.strip()

    def lines(self) -> list[str]:
        """Return stdout as a list of lines."""
        return self.value.splitlines()

    def has_content(self, content: str) -> bool:
        """Check if stdout contains specific content."""
        return content in self.value


class StdError(LogOutput):
    """Standard error wrapper with stderr-specific behavior."""

    def is_empty(self) -> bool:
        """Check if stderr is empty."""
        return not self.value or not self.value.strip()

    def lines(self) -> list[str]:
        """Return stderr as a list of lines."""
        return self.value.splitlines()

    def has_error(self, error_pattern: str) -> bool:
        """Check if stderr contains specific error pattern."""
        return error_pattern.lower() in self.value.lower()

    def is_error(self) -> bool:
        """Check if there is any error output."""
        return bool(self.value and self.value.strip())


class Suggestion(BaseModel):
    """Actionable suggestion wrapper."""

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

    def __eq__(self, other: object) -> bool:
        if isinstance(other, Suggestion):
            return self.value == other.value
        if isinstance(other, str):
            return self.value == other
        return NotImplemented


class BooleanVO(BaseModel):
    """General boolean wrapper."""

    model_config = ConfigDict(frozen=True)
    value: bool

    @model_validator(mode="before")
    @classmethod
    def coerce_from_primitive(cls, data: object) -> object:
        if isinstance(data, bool):
            return {"value": data}
        if isinstance(data, dict) and "value" in data:
            val = data["value"]
            if hasattr(val, "value") and isinstance(val.value, bool):
                data["value"] = val.value
        elif hasattr(data, "value") and isinstance(data.value, bool):
            return {"value": data.value}
        return data

    def __bool__(self) -> bool:
        return self.value

    def __str__(self) -> str:
        return str(self.value)

    def __eq__(self, other: object) -> bool:
        if isinstance(other, BooleanVO):
            return self.value == other.value
        if isinstance(other, bool):
            return self.value == other
        return NotImplemented


class MetadataVO(BaseModel):
    """Generic metadata dictionary wrapper."""

    model_config = ConfigDict(frozen=True)
    value: dict[str, Any] = Field(default_factory=dict)

    @model_validator(mode="before")
    @classmethod
    def coerce_from_primitive(cls, data: object) -> object:
        if isinstance(data, dict) and "value" not in data:
            return {"value": data}
        return data

    def __getitem__(self, key: str) -> Any:
        return self.value[key]

    def items(self):
        """Return the items of the underlying dictionary."""
        return self.value.items()

    def get(self, key: str, default: object = None) -> Any:
        return self.value.get(key, default)

    def dict_copy(
        self,
        *,
        include: object = None,
        exclude: object = None,
        update: dict[str, Any] | None = None,
        deep: bool = False,
    ) -> dict[str, Any]:
        """Return a copy of the underlying dictionary."""
        return self.value.copy()

    def update(self, other: dict[str, Any]) -> None:
        """Update the underlying dictionary."""
        self.value.update(other)

    def __eq__(self, other: object) -> bool:
        if isinstance(other, MetadataVO):
            return self.value == other.value
        if isinstance(other, dict):
            return self.value == other
        return NotImplemented


class ClassPath(BaseModel):
    """Python class path string wrapper."""

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

    def __eq__(self, other: object) -> bool:
        if isinstance(other, ClassPath):
            return self.value == other.value
        if isinstance(other, str):
            return self.value == other
        return NotImplemented


class DescriptionVO(BaseModel):
    """Generic description text wrapper."""

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

    def __eq__(self, other: object) -> bool:
        if isinstance(other, DescriptionVO):
            return self.value == other.value
        if isinstance(other, str):
            return self.value == other
        return NotImplemented


MetadataVO.model_rebuild()
LogOutput.model_rebuild()
