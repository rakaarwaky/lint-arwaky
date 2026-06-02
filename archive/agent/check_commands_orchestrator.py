"""CheckCommandsOrchestrator — Implementation of CheckCommandsAggregate (Agent Logic)."""

from ..contract import CheckCommandsAggregate, ServiceContainerAggregate
from ..taxonomy import FilePath, ComplianceStatus, GovernanceReport


class CheckCommandsOrchestrator(CheckCommandsAggregate):
    """Orchestrator that handles check-related domain logic for the agent."""

    def __init__(self, container: ServiceContainerAggregate) -> None:
        super().__init__(container=container)

    async def check(
        self, path: FilePath, git_diff: ComplianceStatus
    ) -> GovernanceReport:
        """
        Execute check logic.
        Note: The surface currently expects a specific complex output for git-diff.
        We will return the main report, and if it's git-diff, we ensure the report
        contains the aggregated results.
        """
        if bool(git_diff):
            # Resolve the specific project container
            proj_container = self.container.get_for_path(str(path))
            # run_git_diff_analysis returns (main_report, per_file_reports)
            main_report, _ = await proj_container.run_git_diff_analysis(path)
            return main_report or GovernanceReport(results=[], score=0.0)
        else:
            proj_container = self.container.get_for_path(str(path))
            return await proj_container.run_analysis(path)

    async def scan(self, path: FilePath) -> GovernanceReport:
        """Logic: Full deep scan."""
        return await self.check(path, ComplianceStatus(value=False))

    async def run_git_diff(self, path: FilePath):
        """Specific logic for git-diff that returns detailed per-file reports."""
        proj_container = self.container.get_for_path(str(path))
        return await proj_container.run_git_diff_analysis(path)
