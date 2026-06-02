from pydantic import BaseModel, ConfigDict, field_validator, model_validator, Field
from .lint_position_vo import LineNumber, ColumnNumber
from .file_path_vo import FilePath
from .path_collection_vo import PatternList


class ScopeRef(BaseModel):
    """Code scope reference (function, class, module)."""

    model_config = ConfigDict(frozen=True)
    name: str
    kind: str = "function"
    file: FilePath | str = ""
    start_line: LineNumber | int = 0
    end_line: LineNumber | int = 0

    @model_validator(mode="before")
    @classmethod
    def coerce_fields(cls, data: object) -> object:
        if isinstance(data, dict):
            if "file" in data and isinstance(data["file"], str):
                data["file"] = FilePath(value=data["file"])
            if "start_line" in data and isinstance(data["start_line"], int):
                data["start_line"] = LineNumber(value=data["start_line"])
            if "end_line" in data and isinstance(data["end_line"], int):
                data["end_line"] = LineNumber(value=data["end_line"])
        return data

    @field_validator("name")
    @classmethod
    def check_non_empty(cls, v: str) -> str:
        if not v or not v.strip():
            raise ValueError("Scope name cannot be empty")
        return v.strip()

    def __str__(self) -> str:
        if self.file:
            prefix = f"{self.kind} " if self.kind else ""
            return f"{prefix}{self.name} in {self.file}"
        if self.kind:
            return f"{self.kind} {self.name}"
        return self.name

    @property
    def has_range(self) -> bool:
        return int(self.start_line) > 0 and int(self.end_line) > 0


class Location(BaseModel):
    """File location with optional description."""

    model_config = ConfigDict(frozen=True)
    file: FilePath | str = ""
    line: LineNumber = Field(default_factory=lambda: LineNumber(value=0))
    column: ColumnNumber = Field(default_factory=lambda: ColumnNumber(value=0))
    description: str = ""

    @model_validator(mode="before")
    @classmethod
    def coerce_from_primitive(cls, data: object) -> object:
        if isinstance(data, dict):
            if "file" in data and isinstance(data["file"], str) and data["file"].strip():
                data["file"] = FilePath(value=data["file"])
            if "line" in data and isinstance(data["line"], int):
                data["line"] = LineNumber(value=data["line"])
            if "column" in data and isinstance(data["column"], int):
                data["column"] = ColumnNumber(value=data["column"])
        return data

    def __str__(self) -> str:
        parts = []
        if str(self.file):
            parts.append(str(self.file))
        if int(self.line) > 0:
            parts.append(str(self.line))
            if int(self.column) > 0:
                parts[-1] += f":{self.column}"
        result = ":".join(parts) if parts else "unknown"
        if self.description:
            result += f" — {self.description}"
        return result


class LocationList(BaseModel):
    """List of Location objects."""

    model_config = ConfigDict(frozen=False)
    values: list[Location] = Field(default_factory=list)

    def __iter__(self):
        return iter(self.values)

    def __len__(self):
        return len(self.values)

    def append(self, item: Location) -> None:
        self.values.append(item)

    def extend(self, other: "LocationList"):
        self.values.extend(other.values)


class ViolationConstraint(BaseModel):
    """Validation constraint descriptor."""

    model_config = ConfigDict(frozen=True)
    rule: str
    min_value: str = ""
    max_value: str = ""

    def __str__(self) -> str:
        parts = [self.rule]
        if self.min_value or self.max_value:
            range_str = (
                f"{self.min_value}..{self.max_value}"
                if self.min_value and self.max_value
                else self.min_value or self.max_value
            )
            parts.append(f"(must be {range_str})")
        return " ".join(parts)


class CommandArgs(BaseModel):
    """Command line arguments list."""

    model_config = ConfigDict(frozen=True)
    args: PatternList = Field(default_factory=PatternList)

    @field_validator("args", mode="before")
    @classmethod
    def validate_args(cls, v):
        if isinstance(v, list):
            return PatternList(values=v)
        return v

    def __str__(self) -> str:
        return " ".join(self.args.values or [])

    def __len__(self) -> int:
        return len(self.args.values or [])


class ScopeBounds(BaseModel):
    """Start and end line numbers of a scope."""

    model_config = ConfigDict(frozen=True)
    start: LineNumber | None = None
    end: LineNumber | None = None
