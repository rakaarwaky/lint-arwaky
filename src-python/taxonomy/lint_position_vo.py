"""position_vo — Source code position value objects."""

from pydantic import BaseModel, ConfigDict, field_validator, model_validator, Field


class LineNumber(BaseModel):
    """Source code line identifier."""

    model_config = ConfigDict(frozen=True)
    value: int

    @model_validator(mode="before")
    @classmethod
    def coerce_from_primitive(cls, data: object) -> object:
        if isinstance(data, int):
            return {"value": data}
        return data

    @field_validator("value")
    @classmethod
    def check_positive(cls, v: int) -> int:
        if v < 0:
            raise ValueError(f"Line number must be non-negative, got {v}")
        return v

    def __str__(self) -> str:
        return str(self.value)

    def __int__(self) -> int:
        return self.value


class ColumnNumber(BaseModel):
    """Source code column identifier."""

    model_config = ConfigDict(frozen=True)
    value: int

    @model_validator(mode="before")
    @classmethod
    def coerce_from_primitive(cls, data: object) -> object:
        if isinstance(data, int):
            return {"value": data}
        return data

    @field_validator("value")
    @classmethod
    def check_positive(cls, v: int) -> int:
        if v < 0:
            raise ValueError(f"Column number must be non-negative, got {v}")
        return v

    def __str__(self) -> str:
        return str(self.value)

    def __int__(self) -> int:
        return self.value


class Position(BaseModel):
    """Source file location."""

    model_config = ConfigDict(frozen=True)
    line: LineNumber
    column: ColumnNumber = Field(default_factory=lambda: ColumnNumber(value=0))

    def __str__(self) -> str:
        if int(self.column) > 0:
            return f"{self.line}:{self.column}"
        return str(self.line)
