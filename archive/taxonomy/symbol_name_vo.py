"""symbol_name_vo — Symbol name value objects."""

from pydantic import BaseModel, ConfigDict, field_validator, model_validator


class SymbolName(BaseModel):
    """Code symbol identifier (function, class, variable)."""

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
    def check_valid_symbol(cls, v: str) -> str:
        if not v or not v.strip():
            raise ValueError("Symbol name cannot be empty")
        return v.strip()

    def __str__(self) -> str:
        return self.value

    def __hash__(self) -> int:
        return hash(self.value)

    def __eq__(self, other: object) -> bool:
        if isinstance(other, SymbolName):
            return self.value == other.value
        if isinstance(other, str):
            return self.value == other
        return NotImplemented


class NameVariants(BaseModel):
    """List of naming variants for a symbol."""

    model_config = ConfigDict(frozen=True)
    values: list[SymbolName]

    def __iter__(self):
        return iter(self.values)
