"""semantic_tracer_error — Semantic analysis domain error types."""

from pydantic import BaseModel, ConfigDict

from .error_code_vo import ErrorCode
from .error_value_vo import (
    ErrorMessage,
    Cause,
)
from .file_path_vo import FilePath


class SemanticError(BaseModel):
    """General failure during semantic code analysis."""

    model_config = ConfigDict(frozen=True)

    path: FilePath | None = None
    message: ErrorMessage
    error_code: ErrorCode | None = None
    cause: Cause | None = None

    def __str__(self):
        target = f" on {self.path}" if self.path else ""
        code = f" [{self.error_code}]" if self.error_code else ""
        return f"Semantic Error{target}{code}: {self.message}"


class ScopeResolutionError(SemanticError):
    """Failed to resolve or identify code scope (class/function)."""
    pass


class CallChainError(SemanticError):
    """Failed to trace or build a call graph/chain."""
    pass
