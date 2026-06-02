"""file_suffix_vo — Value objects for file suffixes and policies."""

from pydantic import BaseModel, ConfigDict, model_validator


class SuffixVO(BaseModel):
    """File suffix wrapper (e.g., '.py', '_vo.py')."""

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


class SuffixPolicyVO(BaseModel):
    """Architecture suffix enforcement policy."""

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

    @property
    def is_flexible(self) -> bool:
        return self.value == "flexible"

    @property
    def is_strict(self) -> bool:
        return self.value == "strict"
