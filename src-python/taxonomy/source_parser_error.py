"""source_parser_error — Source parsing domain error types."""

from pydantic import BaseModel, ConfigDict

from .error_code_vo import ErrorCode
from .error_value_vo import (
    ErrorMessage,
    Cause,
)
from .lint_position_vo import (
    LineNumber,
    ColumnNumber,
)
from .file_path_vo import FilePath


class SourceParserError(BaseModel):
    """General failure during source code parsing."""

    model_config = ConfigDict(frozen=True)

    path: FilePath
    message: ErrorMessage
    error_code: ErrorCode | None = None
    cause: Cause | None = None

    def __str__(self):
        code = f" [{self.error_code}]" if self.error_code else ""
        return f"Parser Error on {self.path}{code}: {self.message}"


class SyntaxErrorVO(SourceParserError):
    """Source code contains invalid syntax."""

    line: LineNumber | None = None
    column: ColumnNumber | None = None

    def __str__(self):
        pos = f" at {self.line}:{self.column}" if self.line else ""
        return f"Syntax Error on {self.path}{pos}: {self.message}"
