from pydantic import BaseModel, ConfigDict, Field, model_validator
from .symbol_name_vo import SymbolName


class SymbolNameList(BaseModel):
    """Collection of symbol names."""

    model_config = ConfigDict(frozen=True)

    @model_validator(mode="before")
    @classmethod
    def coerce_from_list(cls, data: object) -> object:
        if isinstance(data, list):
            return {"values": data}
        return data

    values: list[SymbolName] = Field(default_factory=list)

    def __iter__(self):
        return iter(self.values)

    def __len__(self) -> int:
        return len(self.values)


class ImportNameList(BaseModel):
    """Collection of imported module/symbol names."""

    model_config = ConfigDict(frozen=True)

    @model_validator(mode="before")
    @classmethod
    def coerce_from_list(cls, data: object) -> object:
        if isinstance(data, list):
            return {"values": data}
        return data

    values: list[SymbolName] = Field(default_factory=list)

    def __len__(self) -> int:
        return len(self.values)


class PrimitiveTypeList(BaseModel):
    """List of primitive type names."""

    model_config = ConfigDict(frozen=True)

    @model_validator(mode="before")
    @classmethod
    def coerce_from_list(cls, data: object) -> object:
        if isinstance(data, list):
            return {"values": data}
        return data

    values: list[SymbolName] = Field(default_factory=list)

    def __contains__(self, item: str) -> bool:
        return any((v.value == item for v in self.values))

    def __iter__(self):
        return iter(self.values)

    def __len__(self) -> int:
        return len(self.values)


class CallChainList(BaseModel):
    """List of call site strings (or symbol locations)."""

    model_config = ConfigDict(frozen=True)

    @model_validator(mode="before")
    @classmethod
    def coerce_from_list(cls, data: object) -> object:
        if isinstance(data, list):
            return {"values": data}
        return data

    values: list[SymbolName] = Field(default_factory=list)

    def __iter__(self):
        return iter(self.values)

    def __len__(self) -> int:
        return len(self.values)


CORE_PRIMITIVE_TYPES = ["str", "int", "float"]

PRIMITIVE_TYPE_LIST = PrimitiveTypeList(
    values=[SymbolName(value=t) for t in CORE_PRIMITIVE_TYPES]
)
