"""analysis_execution_orchestrator — Implementation of the analysis orchestration domain contract."""

from __future__ import annotations
from ..taxonomy import FilePath, GovernanceReport
from ..contract import (
    AnalysisOrchestratorAggregate,
    ServiceContainerAggregate,
    LintPipelineOrchestratorAggregate,
)


class AnalysisOrchestrator(AnalysisOrchestratorAggregate):
    """Agent orchestrator that coordinates analysis-specific domain logic."""

    @property
    def _INTERFACE_CONTAINER(self):
        return ServiceContainerAggregate

    @property
    def _lint_pipeline(self) -> LintPipelineOrchestratorAggregate:
        """ARCHITECTURAL COMMITMENT: Resolve the pipeline via the container."""
        return self.container.get(LintPipelineOrchestratorAggregate)

    async def run(self, path: FilePath) -> GovernanceReport:
        """Execute full project analysis via the linting pipeline."""
        return await self._lint_pipeline.run(path)

    async def get_complexity(self, path: FilePath) -> GovernanceReport:
        """Execute complexity analysis by filtering full report for radon results."""
        report = await self.run(path)
        report.results = [r for r in report.results if str(r.source or "") == "radon"]
        return report

    async def get_duplicates(self, path: FilePath) -> GovernanceReport:
        """Execute duplication analysis by filtering full report for duplicates results."""
        report = await self.run(path)
        report.results = [
            r for r in report.results if str(r.source or "") == "duplicates"
        ]
        return report

    async def get_trends(self, path: FilePath) -> GovernanceReport:
        """Execute quality trends analysis by filtering full report for trends results."""
        report = await self.run(path)
        report.results = [r for r in report.results if str(r.source or "") == "trends"]
        return report

    async def get_dependencies(self, path: FilePath) -> GovernanceReport:
        """Execute dependency vulnerability analysis by filtering full report for pip-audit results."""
        report = await self.run(path)
        report.results = [
            r for r in report.results if str(r.source or "") == "pip-audit"
        ]
        return report
