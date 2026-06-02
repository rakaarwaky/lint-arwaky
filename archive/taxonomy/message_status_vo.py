"""message_status_vo — Message and status value objects."""

from pydantic import BaseModel, ConfigDict, field_validator, model_validator


class LintMessage(BaseModel):
    """Lint finding description."""

    model_config = ConfigDict(frozen=True)
    value: str

    @model_validator(mode="before")
    @classmethod
    def handle_error_message(cls, data: object) -> object:
        if isinstance(data, dict) and "value" in data:
            val = data["value"]
            if hasattr(val, "value") and type(val).__name__ == "ErrorMessage":
                data["value"] = str(val)
        elif hasattr(data, "value") and type(data).__name__ == "ErrorMessage":
            return {"value": str(data)}
        return data

    @field_validator("value")
    @classmethod
    def check_non_empty(cls, v: str) -> str:
        if not v or not v.strip():
            raise ValueError("Lint message cannot be empty")
        return v.strip()

    def __str__(self) -> str:
        return self.value


class ComplianceStatus(BaseModel):
    """Binary compliance indicator."""

    model_config = ConfigDict(frozen=True)
    value: bool

    @model_validator(mode="before")
    @classmethod
    def coerce_from_primitive(cls, data: object) -> object:
        if isinstance(data, bool):
            return {"value": data}
        return data

    def __str__(self) -> str:
        return "PASS" if self.value else "FAIL"

    def __bool__(self) -> bool:
        return self.value


class Count(BaseModel):
    """Integer counter for lint metrics."""

    model_config = ConfigDict(frozen=True)
    value: int

    @model_validator(mode="before")
    @classmethod
    def coerce_from_primitive(cls, data: object) -> object:
        if isinstance(data, int):
            return {"value": data}
        if isinstance(data, dict) and "value" in data:
            val = data["value"]
            if hasattr(val, "value") and isinstance(val.value, int):
                data["value"] = val.value
        elif hasattr(data, "value") and isinstance(data.value, int):
            return {"value": data.value}
        return data

    @field_validator("value")
    @classmethod
    def check_non_negative(cls, v: int) -> int:
        if v < 0:
            raise ValueError(f"Count must be non-negative, got {v}")
        return v

    def __str__(self) -> str:
        return str(self.value)

    def __int__(self) -> int:
        return self.value
