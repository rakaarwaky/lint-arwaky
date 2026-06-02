"""score_format_vo — Score and format value objects."""

from pydantic import BaseModel, ConfigDict, field_validator, model_validator
from .lint_severity_vo import Severity


class Score(BaseModel):
    """Architecture compliance score (unbounded, default 100.0)."""

    model_config = ConfigDict(frozen=True)
    value: float

    @model_validator(mode="before")
    @classmethod
    def coerce_from_primitive(cls, data: object) -> object:
        if isinstance(data, (int, float)):
            return {"value": float(data)}
        if isinstance(data, dict) and "value" in data:
            val = data["value"]
            if hasattr(val, "value") and isinstance(val.value, (int, float)):
                data["value"] = float(val.value)
        elif hasattr(data, "value") and isinstance(data.value, (int, float)):
            return {"value": float(data.value)}
        return data

    @field_validator("value")
    @classmethod
    def check_range(cls, v: float) -> float:
        if v > 100.0:
            raise ValueError(f"Score must be at most 100.0, got {v}")
        return v

    def __str__(self) -> str:
        return f"{self.value:.1f}"

    def __float__(self) -> float:
        return self.value

    def is_passing(self, threshold: "Score") -> bool:
        """Check if score meets the provided threshold."""
        return self.value >= threshold.value

    @property
    def is_perfect(self) -> bool:
        return self.value >= 100.0

    def deduct(self, severity: Severity) -> "Score":
        new_val = self.value - severity.score_impact
        return Score(value=new_val)


class FileFormat(BaseModel):
    """Report output format."""

    model_config = ConfigDict(frozen=True)
    name: str

    def __str__(self) -> str:
        return self.name

    @property
    def is_structured(self) -> bool:
        return self.name in ("json", "sarif", "junit")


FORMAT_TEXT = FileFormat(name="text")
FORMAT_JSON = FileFormat(name="json")
FORMAT_SARIF = FileFormat(name="sarif")
FORMAT_JUNIT = FileFormat(name="junit")
ALL_FORMATS = (FORMAT_TEXT, FORMAT_JSON, FORMAT_SARIF, FORMAT_JUNIT)
