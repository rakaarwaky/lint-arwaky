"""time_duration_vo — Time and duration value objects."""

from pydantic import BaseModel, ConfigDict, field_validator, model_validator


class Duration(BaseModel):
    """Time duration in milliseconds."""

    model_config = ConfigDict(frozen=True)
    value: float

    @model_validator(mode="before")
    @classmethod
    def coerce_from_primitive(cls, data: object) -> object:
        if isinstance(data, (int, float)):
            return {"value": float(data)}
        return data

    @field_validator("value")
    @classmethod
    def check_non_negative(cls, v: float) -> float:
        if v < 0:
            raise ValueError(f"Duration must be non-negative, got {v}")
        return v

    def __str__(self) -> str:
        return f"{self.value:.2f}ms"

    def __float__(self) -> float:
        return self.value


class Timeout(BaseModel):
    """Timeout in seconds."""

    model_config = ConfigDict(frozen=True)
    value: float

    @model_validator(mode="before")
    @classmethod
    def coerce_from_primitive(cls, data: object) -> object:
        if isinstance(data, (int, float)):
            return {"value": float(data)}
        return data

    @field_validator("value")
    @classmethod
    def check_positive(cls, v: float) -> float:
        if v <= 0:
            raise ValueError(f"Timeout must be positive, got {v}")
        return v

    def __str__(self) -> str:
        return f"{self.value}s"

    def __float__(self) -> float:
        return self.value


class Timestamp(BaseModel):
    """UTC timestamp string wrapper."""

    model_config = ConfigDict(frozen=True)
    value: str

    @field_validator("value", mode="before")
    @classmethod
    def validate_timestamp(cls, v):
        from datetime import datetime, timezone

        if not v:
            return datetime.now(timezone.utc).isoformat()
        return v

    def __str__(self) -> str:
        return self.value

    def __eq__(self, other: object) -> bool:
        if isinstance(other, Timestamp):
            return self.value == other.value
        if isinstance(other, str):
            return self.value == other
        return NotImplemented
