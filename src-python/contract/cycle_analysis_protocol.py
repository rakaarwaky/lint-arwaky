"""cycle_analysis_protocol — Interface for cycle analysis capability."""

from abc import ABC, abstractmethod
from ..taxonomy import FilePath, LintResultList, ComplianceStatus, AdapterName


class ICycleAnalysisProtocol(ABC):
    """Interface for detecting circular imports and dependency cycles (Protocol)."""

    @abstractmethod
    def scan(self, path: FilePath) -> LintResultList:
        """Scan path (file or directory) for circular import violations."""
        ...

    @abstractmethod
    def apply_fix(self, path: FilePath) -> ComplianceStatus:
        """Circular import violations require manual architectural refactoring."""
        ...

    @abstractmethod
    def name(self) -> AdapterName:
        """Return the adapter name."""
        ...
