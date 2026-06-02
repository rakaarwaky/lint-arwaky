from abc import ABC, abstractmethod
from ..taxonomy import FilePath, ComplianceStatus, LintResultList

class ArchCoordinatorAggregate(ABC):
    """AGGREGATE: Orchestrates architectural compliance across the codebase."""
    @abstractmethod
    async def check_compliance(self, path: FilePath) -> ComplianceStatus:
        """Button: Perform full compliance check for a project path."""
        ...

    @abstractmethod
    async def scan(self, path: FilePath) -> LintResultList:
        """Bridge: Perform architectural scan for the linter pipeline."""
        ...

    @abstractmethod
    async def apply_fix(self, path: FilePath) -> ComplianceStatus:
        """Bridge: Apply architectural fixes."""
        ...
