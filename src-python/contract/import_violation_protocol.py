"""import_violation_protocol — Interface for import violation analysis capability."""

from abc import ABC, abstractmethod
from ..taxonomy import FilePath, LintResultList, ComplianceStatus, AdapterName


class IImportViolationProtocol(ABC):
    """Interface for enforcing cross-layer import restrictions (Protocol)."""

    @abstractmethod
    def scan(self, path: FilePath) -> LintResultList:
        """Scan path (file or directory) for cross-layer import violations."""
        ...

    @abstractmethod
    def apply_fix(self, path: FilePath) -> ComplianceStatus:
        """Import violations require manual architectural refactoring."""
        ...

    @abstractmethod
    def name(self) -> AdapterName:
        """Return the adapter name."""
        ...
