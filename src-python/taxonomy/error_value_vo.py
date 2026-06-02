"""error_value_vo — Error and value wrapper value objects."""

from pydantic import BaseModel, ConfigDict, model_validator


class ErrorMessage(BaseModel):
    """Generic error message wrapper."""

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
        if isinstance(other, ErrorMessage):
            return self.value == other.value
        if isinstance(other, str):
            return self.value == other
        return NotImplemented


class ExpectedValue(BaseModel):
    """Wrapper for expected configuration value."""

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
        if isinstance(other, ExpectedValue):
            return self.value == other.value
        if isinstance(other, str):
            return self.value == other
        return NotImplemented


class ActualValue(BaseModel):
    """Wrapper for actual configuration value."""

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
        if isinstance(other, ActualValue):
            return self.value == other.value
        if isinstance(other, str):
            return self.value == other
        return NotImplemented


class FieldName(BaseModel):
    """Wrapper for configuration field name."""

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
        if isinstance(other, FieldName):
            return self.value == other.value
        if isinstance(other, str):
            return self.value == other
        return NotImplemented


class Constraint(BaseModel):
    """Wrapper for configuration constraint description."""

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
        if isinstance(other, Constraint):
            return self.value == other.value
        if isinstance(other, str):
            return self.value == other
        return NotImplemented


class ExitCode(BaseModel):
    """Process exit code wrapper."""

    model_config = ConfigDict(frozen=True)
    value: int

    @model_validator(mode="before")
    @classmethod
    def coerce_from_primitive(cls, data: object) -> object:
        if isinstance(data, int):
            return {"value": data}
        return data

    def __str__(self) -> str:
        return str(self.value)

    def __eq__(self, other: object) -> bool:
        if isinstance(other, ExitCode):
            return self.value == other.value
        if isinstance(other, int):
            return self.value == other
        return NotImplemented

    def __int__(self) -> int:
        return self.value


class Cause(BaseModel):
    """Error cause wrapper."""

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
        if isinstance(other, Cause):
            return self.value == other.value
        if isinstance(other, str):
            return self.value == other
        return NotImplemented


class ModuleName(BaseModel):
    """Python module name wrapper."""

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
        if isinstance(other, ModuleName):
            return self.value == other.value
        if isinstance(other, str):
            return self.value == other
        return NotImplemented


class PrimitiveTypeName(BaseModel):
    """Native primitive type name wrapper."""

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
        if isinstance(other, PrimitiveTypeName):
            return self.value == other.value
        if isinstance(other, str):
            return self.value == other
        return NotImplemented
