from pydantic import BaseModel, ConfigDict, Field, model_validator
from .file_path_vo import FilePath


class FilePathList(BaseModel):
    """Collection of file paths."""

    model_config = ConfigDict(frozen=True)

    @model_validator(mode="before")
    @classmethod
    def coerce_from_list(cls, data: object) -> object:
        if isinstance(data, list):
            return {"values": data}
        return data

    values: list[FilePath] = []

    def __iter__(self):
        return iter(self.values)

    def __len__(self) -> int:
        return len(self.values)


class FilePathSet(BaseModel):
    """Set of unique file paths."""

    model_config = ConfigDict(frozen=True)

    @model_validator(mode="before")
    @classmethod
    def coerce_from_collection(cls, data: object) -> object:
        if isinstance(data, (list, set)):
            return {"values": set(data)}
        return data

    values: set[FilePath] = Field(default_factory=set)

    def __iter__(self):
        return iter(self.values)

    def __contains__(self, item: object) -> bool:
        return item in self.values

    def __len__(self) -> int:
        return len(self.values)


class RenamedFile(BaseModel):
    """Represents a file rename operation."""

    model_config = ConfigDict(frozen=True)
    old_path: FilePath
    new_path: FilePath


class RenamedFileList(BaseModel):
    """Collection of renamed files."""

    model_config = ConfigDict(frozen=True)

    @model_validator(mode="before")
    @classmethod
    def coerce_from_list(cls, data: object) -> object:
        if isinstance(data, list):
            return {"values": data}
        return data

    values: list[RenamedFile] = []

    def __iter__(self):
        return iter(self.values)

    def __len__(self) -> int:
        return len(self.values)


class PatternList(BaseModel):
    """List of string patterns."""

    model_config = ConfigDict(frozen=True)

    @model_validator(mode="before")
    @classmethod
    def coerce_from_list(cls, data: object) -> object:
        if data is None:
            return {"values": None}
        if isinstance(data, list):
            return {"values": data}
        return data

    values: list[str] | None = Field(default_factory=list)

    def __iter__(self):
        return iter(self.values or [])

    def __contains__(self, item: str) -> bool:
        if self.values is None:
            return False
        return item in self.values

    def __len__(self) -> int:
        return len(self.values or [])
