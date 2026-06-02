"""Infrastructure adapter that wraps the architectural compliance capability."""

from __future__ import annotations

from ..taxonomy import (
    AdapterName,
    ComplianceStatus,
    FilePath,
    LintResultList,
    AdapterError,
    ScanError,
)
from ..contract import ILinterAdapterPort, IArchCompliancePort


class ArchComplianceAdapter(ILinterAdapterPort):
    """Adapter that bridges architectural compliance to the linter pipeline.
    
    Implements ILinterAdapterPort by delegating to an IArchCompliancePort.
    """

    def __init__(self, coordinator: IArchCompliancePort):
        self.coordinator = coordinator

    def name(self) -> AdapterName:
        return AdapterName(value="architecture")

    async def scan(self, path: FilePath) -> LintResultList | ScanError | AdapterError:
        """Performs architectural scan using the compliance coordinator."""
        try:
            return await self.coordinator.scan(path)
        except Exception as e:
            return AdapterError(adapter_name="architecture", message=f"Architecture scan failed: {str(e)}")

    async def apply_fix(self, path: FilePath) -> ComplianceStatus | AdapterError:
        """Execute architectural fix."""
        try:
            return await self.coordinator.apply_fix(path)
        except Exception as e:
            return AdapterError(adapter_name="architecture", message=f"Architecture fix failed: {str(e)}")
