"""arch_compliance_coordinator — Internal coordinator to treat architecture checks as a linter."""

from ..taxonomy import (
    AdapterName,
    ComplianceStatus,
    FilePath,
    LintResultList,
)

from ..contract import (
    IArchComplianceProtocol,
    ArchComplianceCoordinatorAggregate,
    ILinterAdapterPort,
    IArchCompliancePort,
)


class ArchComplianceCoordinator(ArchComplianceCoordinatorAggregate, IArchCompliancePort):
    """Coordinator bridging multiple IArchComplianceProtocol orchestrators.

    Coordinates across 1+ compliance orchestrators to produce a unified scan result.
    Each orchestrator executes independently; this coordinator aggregates their outputs
    so the external linter pipeline sees a single compliance surface.

    Inherits from ArchComplianceCoordinatorAggregate (contract) + IArchCompliancePort.
    """

    @property
    def _INTERFACE_PORT(self):
        # Implementation detail: we import ILinterAdapterPort at top-level now.
        return ILinterAdapterPort

    @property
    def _INTERFACE_CAPABILITY(self):
        return IArchComplianceProtocol

    def __init__(
        self,
        compliance_orchestrator: IArchComplianceProtocol,
        additional_orchestrators: list[IArchComplianceProtocol] | None = None,
    ):
        # The aggregate contract (ArchCoordinatorAggregate) is a plain ABC — no Pydantic.
        super().__init__()
        self._additional_orchestrators: list[IArchComplianceProtocol] = (
            additional_orchestrators or []
        )
        self._orchestrators: list[IArchComplianceProtocol] = [
            compliance_orchestrator
        ] + self._additional_orchestrators

    def name(self) -> AdapterName:
        return AdapterName(value="architecture")

    async def check_compliance(self, path: FilePath) -> ComplianceStatus:
        """Fulfills ArchCoordinatorAggregate contract. Delegates to scan."""
        result = await self.scan(path)
        return ComplianceStatus(value=len(result.values) == 0)

    async def scan(self, path: FilePath) -> LintResultList:
        """Performs architectural scan using the compliance analyzer defined in the Aggregate."""
        results = LintResultList(values=[])
        for orchestrator in self._orchestrators:
            results.values.extend(orchestrator.execute(path).values)
        return results

    async def apply_fix(self, path: FilePath) -> ComplianceStatus:
        """Architecture fixes are not supported automatically yet."""
        return ComplianceStatus(value=False)
