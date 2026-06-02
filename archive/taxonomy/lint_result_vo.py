from pydantic import BaseModel, ConfigDict, Field, field_validator
from .lint_severity_vo import Severity
from .error_code_vo import ErrorCode
from .lint_position_vo import Position, LineNumber, ColumnNumber
from .message_status_vo import LintMessage
from .file_path_vo import FilePath
from .adapter_name_vo import AdapterName
from .layer_content_vo import Identity
from .lint_domain_vo import ScopeRef, Location, LocationList


class LintResult(BaseModel):
    """A single lint finding."""

    model_config = ConfigDict(frozen=False)
    file: FilePath
    line: LineNumber
    column: ColumnNumber = Field(default_factory=lambda: ColumnNumber(value=0))
    code: ErrorCode = Field(default_factory=lambda: ErrorCode(code="UNKNOWN"))
    message: LintMessage = Field(
        default_factory=lambda: LintMessage(value="No message provided")
    )
    source: AdapterName | None = None
    severity: Severity = Severity.MEDIUM
    enclosing_scope: ScopeRef | None = None
    related_locations: LocationList = Field(default_factory=LocationList)

    @field_validator("file", mode="before")
    @classmethod
    def validate_file(cls, v):
        if isinstance(v, str):
            return FilePath(value=v)
        return v

    @field_validator("line", mode="before")
    @classmethod
    def validate_line(cls, v):
        if isinstance(v, int):
            return LineNumber(value=v)
        return v

    @field_validator("column", mode="before")
    @classmethod
    def validate_column(cls, v):
        if isinstance(v, int):
            return ColumnNumber(value=v)
        return v

    @field_validator("code", mode="before")
    @classmethod
    def validate_code(cls, v):
        if isinstance(v, str):
            return ErrorCode(code=v)
        return v

    @field_validator("message", mode="before")
    @classmethod
    def validate_message(cls, v):
        if isinstance(v, str):
            return LintMessage(value=v)
        return v

    @field_validator("source", mode="before")
    @classmethod
    def validate_source(cls, v):
        if isinstance(v, str):
            return AdapterName(value=v)
        return v

    @field_validator("related_locations", mode="before")
    @classmethod
    def validate_related_locations(cls, v):
        if isinstance(v, list):
            vals = []
            for item in v:
                if isinstance(item, str):
                    vals.append(Location(description=item))
                else:
                    vals.append(item)
            return LocationList(values=vals)
        return v

    @property
    def position(self) -> Position:
        return Position(line=self.line, column=self.column)

    @property
    def identity(self) -> Identity:
        """Unique key for deduplication."""
        return Identity(value=f"{self.file}:{self.line}:{self.code}:{self.source}")


class LintResultList(BaseModel):
    """List of LintResult objects."""

    model_config = ConfigDict(frozen=False)
    values: list[LintResult] = Field(default_factory=list)

    def __iter__(self):
        return iter(self.values)

    def __len__(self):
        return len(self.values)

    def append(self, item: LintResult) -> None:
        self.values.append(item)
