"""layer_content_vo — Layer, content, and identity value objects."""

from pydantic import BaseModel, ConfigDict, model_validator


class LayerNameVO(BaseModel):
    """Architectural layer identifier."""

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
        if isinstance(other, LayerNameVO):
            return self.value == other.value
        if isinstance(other, str):
            return self.value == other
        return NotImplemented

    def __lt__(self, other: object) -> bool:
        if isinstance(other, LayerNameVO):
            return self.value < other.value
        if isinstance(other, str):
            return self.value < other
        return NotImplemented


class FileContentVO(BaseModel):
    """File content wrapper."""

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
        if isinstance(other, FileContentVO):
            return self.value == other.value
        if isinstance(other, str):
            return self.value == other
        return NotImplemented


class LineContentVO(BaseModel):
    """A single line of source code."""

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
        if isinstance(other, LineContentVO):
            return self.value == other.value
        if isinstance(other, str):
            return self.value == other
        return NotImplemented


class Identity(BaseModel):
    """Unique identifier for deduplication."""

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
        if isinstance(other, Identity):
            return self.value == other.value
        if isinstance(other, str):
            return self.value == other
        return NotImplemented
