"""linter_adapter_port — Port interface for external linting tools.

Infrastructure implements this. Capabilities consume it via DI.
"""

from abc import ABC, abstractmethod

from ..taxonomy import (
    LintResultList,
    FilePath,
    AdapterName,
    ComplianceStatus,
    AdapterError,
    ScanError,
)


class ILinterAdapterPort(ABC):
    """Port interface for external linting tools."""

    @abstractmethod
    async def scan(self, path: FilePath) -> LintResultList | ScanError | AdapterError:
        """Scan the given path and return a list of LintResult."""
        ...

    @abstractmethod
    async def apply_fix(self, path: FilePath) -> ComplianceStatus | AdapterError:
        """Apply automatic fixes to the given path."""
        ...

    @abstractmethod
    def name(self) -> AdapterName:
        """Return the name of the tool (e.g., 'ruff')."""
        ...
