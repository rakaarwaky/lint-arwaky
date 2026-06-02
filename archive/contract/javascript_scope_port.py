"""javascript_scope_port — Port for JS/TS scope detection."""

from abc import ABC, abstractmethod
from ..taxonomy import (
    LineContentVO,
    LineNumber,
    ScopeBounds,
    SymbolName,
    LineContentList,
    SemanticError,
)


class IJSScopeProviderPort(ABC):
    """Port for JS/TS scope detection and bound resolution."""

    @abstractmethod
    def detect_js_scope(self, stripped_line: LineContentVO) -> SymbolName | SemanticError | None:
        """Detect if a line opens a named scope (class, function)."""
        ...

    @abstractmethod
    def find_scope_bounds(
        self, lines: LineContentList, scope_line: LineNumber | None
    ) -> ScopeBounds | SemanticError | None:
        """Find start/end line numbers of enclosing function body."""
        ...
