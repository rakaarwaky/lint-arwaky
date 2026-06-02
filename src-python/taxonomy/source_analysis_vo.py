from pydantic import BaseModel, ConfigDict, Field
from .lint_position_vo import LineNumber, ColumnNumber
from .error_value_vo import ModuleName, PrimitiveTypeName
from .symbol_name_vo import SymbolName


class ImportInfo(BaseModel):
    """Information about a single import statement."""

    model_config = ConfigDict(frozen=True)
    line: LineNumber
    module: ModuleName
    name: SymbolName | None = None


class PrimitiveViolation(BaseModel):
    """Information about a forbidden primitive type usage."""

    model_config = ConfigDict(frozen=True)
    line: LineNumber
    column: ColumnNumber
    type_name: PrimitiveTypeName


class ImportInfoList(BaseModel):
    """List of ImportInfo objects."""

    model_config = ConfigDict(frozen=True)
    values: list[ImportInfo] = Field(default_factory=list)

    def __iter__(self):
        return iter(self.values)

    def __len__(self):
        return len(self.values)

    def append(self, item: ImportInfo) -> None:
        """Append an item to the list."""
        self.values.append(item)


class PrimitiveViolationList(BaseModel):
    """List of PrimitiveViolation objects."""

    model_config = ConfigDict(frozen=True)
    values: list[PrimitiveViolation] = Field(default_factory=list)

    def __iter__(self):
        return iter(self.values)

    def __len__(self):
        return len(self.values)

    def append(self, item: PrimitiveViolation) -> None:
        """Append an item to the list."""
        self.values.append(item)
