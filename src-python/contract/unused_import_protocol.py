"""unused_import_checker_protocol — Protocol for detecting unused imports.

Capabilities implement this (UnusedImportChecker). Agent consumes via DI.
"""

from abc import ABC, abstractmethod
from ..taxonomy import FilePath, ImportNameList


class IUnusedImportProtocol(ABC):
    """Protocol for identifying imports that are not utilized in the code."""

    @abstractmethod
    def find_unused_imports(self, path: FilePath) -> ImportNameList:
        """Return list of imported symbols that are never referenced."""
        ...
