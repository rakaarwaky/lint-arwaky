"""Orchestrator for report and security CLI commands logic."""

from ..contract import ReportCommandsAggregate, ServiceContainerAggregate
from ..taxonomy import FilePath, GovernanceReport, Identity, FileFormat


class ReportCommandsOrchestrator(ReportCommandsAggregate):
    """
    AGENT LAYER ORCHESTRATOR

    Handles the coordination between the surface and the project containers
    for generating reports and security scans.
    """

    container: ServiceContainerAggregate | None = None

    def __init__(self, container: ServiceContainerAggregate):
        super().__init__(container=container)

    def report(
        self, path: FilePath | Identity, output_format: FileFormat | Identity
    ) -> None:
        """
        Logic for generating a report.
        """
        pass

    def security(self, path: FilePath | Identity) -> None:
        """Logic for security scan."""
        pass

    async def run_analysis(self, path: FilePath) -> GovernanceReport:
        """Orchestrate the analysis run."""
        if self.container is None:
            raise RuntimeError("Container not initialized")
        container = self.container.get_for_path(path)
        return await container.run_analysis(path)

    def get_formatted_output(
        self, report_data: GovernanceReport, output_format: FileFormat, path: FilePath
    ) -> str:
        """Get formatted output from the container."""
        if self.container is None:
            raise RuntimeError("Container not initialized")
        container = self.container.get_for_path(path)

        if str(output_format) == "json":
            if hasattr(container, "to_dict"):
                import json

                return json.dumps(container.to_dict(report_data), indent=2)
            return "{}"
        elif output_format == "sarif":
            if hasattr(container, "to_sarif_report"):
                return str(container.to_sarif_report(report_data))
            return ""
        elif output_format == "junit":
            if hasattr(container, "to_junit_report"):
                return str(container.to_junit_report(report_data))
            return ""
        return ""
