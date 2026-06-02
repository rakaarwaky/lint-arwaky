"""scope_boundary_resolver_protocol — Protocol for resolving code scope boundaries.

Capabilities implement this (ScopeBoundaryResolver). Infrastructure/Agent consume via DI.
"""

from abc import ABC, abstractmethod
from ..taxonomy import FilePath, LineNumber, ScopeRef


class IScopeBoundaryResolverProtocol(ABC):
    """Protocol for detecting and resolving function/class boundaries."""

    @abstractmethod
    def resolve_enclosing_scope(
        self, file_path: FilePath, line: LineNumber
    ) -> ScopeRef | None:
        """Return the scope hierarchy enclosing the given line."""
        ...
