from abc import ABC, abstractmethod
from ..taxonomy import (
    FilePath,
    LineNumber,
    SymbolName,
    FileContentVO,
    LineContentVO,
    LintResultList,
    ScopeBounds,
)


class IArchComplianceProtocol(ABC):
    """Abstraction for architectural compliance analysis (Capability Protocol)."""

    def execute(self, path: FilePath) -> LintResultList:
        """Performs architectural scan logic."""
        raise NotImplementedError


class IScopeBoundaryProtocol(ABC):
    """Abstraction for JS/TS scope detection (Capability Protocol)."""

    @abstractmethod
    def detect_js_scope(self, stripped_line: LineContentVO) -> SymbolName | None:
        """Detect if a line opens a named scope."""
        ...

    @abstractmethod
    def find_scope_bounds(
        self, content: FileContentVO, scope_line: LineNumber | None
    ) -> ScopeBounds:
        """Find start/end line numbers of enclosing function body."""
        ...

    @abstractmethod
    def get_enclosing_scope(
        self, file_path: FilePath, line: LineNumber
    ) -> SymbolName | None:
        """Find the nearest enclosing function or class for a given line."""
        ...
