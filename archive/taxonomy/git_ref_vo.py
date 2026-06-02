"""git_ref_vo — Git reference value objects."""

from pydantic import BaseModel, ConfigDict, field_validator, model_validator


class GitRef(BaseModel):
    """Git reference (branch, tag, or special ref)."""

    model_config = ConfigDict(frozen=True)
    value: str

    @model_validator(mode="before")
    @classmethod
    def coerce_from_primitive(cls, data: object) -> object:
        if isinstance(data, str):
            return {"value": data}
        return data

    @field_validator("value")
    @classmethod
    def check_non_empty(cls, v: str) -> str:
        if not v or not v.strip():
            raise ValueError("Git ref cannot be empty")
        return v.strip()

    def __str__(self) -> str:
        return self.value

    def __hash__(self) -> int:
        return hash(self.value)

    def __eq__(self, other: object) -> bool:
        if isinstance(other, GitRef):
            return self.value == other.value
        if isinstance(other, str):
            return self.value == other
        return NotImplemented
