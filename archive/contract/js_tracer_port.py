"""javascript_scope_tracer_port — Port for JS scope tracing."""

from abc import ABC, abstractmethod

from ..taxonomy import FilePath, LineNumber, ScopeRef, SemanticError


class IJSScopeTracerPort(ABC):
    """Port for finding enclosing scopes in JS/TS."""

    @abstractmethod
    def show_enclosing_scope(
        self, file_path: FilePath, line: LineNumber
    ) -> ScopeRef | SemanticError | None:
        """Find the nearest enclosing function or class."""
        ...
