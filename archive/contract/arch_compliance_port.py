"""arch_compliance_port — Port for architectural compliance coordinator."""

from abc import ABC, abstractmethod
from ..taxonomy import FilePath, ComplianceStatus, LintResultList

class IArchCompliancePort(ABC):
    """Port interface for orchestrating architectural compliance."""
    
    @abstractmethod
    async def scan(self, path: FilePath) -> LintResultList:
        """Perform architectural scan."""
        ...

    @abstractmethod
    async def apply_fix(self, path: FilePath) -> ComplianceStatus:
        """Apply architectural fixes."""
        ...
