from pydantic import BaseModel, ConfigDict, Field, model_validator
from .adapter_name_vo import AdapterName
from .lint_status_vo import AdapterMetadata


class AdapterMetadataList(BaseModel):
    """Collection of adapter metadata."""

    model_config = ConfigDict(frozen=True)

    @model_validator(mode="before")
    @classmethod
    def coerce_from_list(cls, data: object) -> object:
        if isinstance(data, list):
            return {"values": data}
        return data

    values: list[AdapterMetadata] = Field(default_factory=list)

    def __iter__(self):
        return iter(self.values)

    def __len__(self) -> int:
        return len(self.values)


class AdapterNameList(BaseModel):
    """Collection of adapter names."""

    model_config = ConfigDict(frozen=True)

    @model_validator(mode="before")
    @classmethod
    def coerce_from_list(cls, data: object) -> object:
        if isinstance(data, list):
            return {"values": data}
        return data

    values: list[AdapterName] = Field(default_factory=list)

    def __iter__(self):
        return iter(self.values)

    def __len__(self) -> int:
        return len(self.values)


class AdapterClassMap(BaseModel):
    """Map of adapter names to their implementation classes."""

    model_config = ConfigDict(frozen=True, arbitrary_types_allowed=True)

    @model_validator(mode="before")
    @classmethod
    def coerce_from_dict(cls, data: object) -> object:
        if isinstance(data, dict):
            return {"values": data}
        return data

    values: dict[AdapterName, type] = Field(default_factory=dict)

    def __getitem__(self, key: AdapterName) -> type:
        return self.values[key]

    def __iter__(self):
        return iter(self.values)

    def __len__(self) -> int:
        return len(self.values)
