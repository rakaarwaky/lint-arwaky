"""content_string_vo — Value object for strings used as content (Agent/Surface layer)."""

from pydantic import BaseModel, ConfigDict, model_validator


class ContentString(BaseModel):
    """VO for content strings, often used for actions or identifiers."""

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

    def __hash__(self) -> int:
        return hash(self.value)

    def __eq__(self, other: object) -> bool:
        if isinstance(other, ContentString):
            return self.value == other.value
        if isinstance(other, str):
            return self.value == other
        return False
