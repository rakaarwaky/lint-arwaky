from typing import Type
import logging
from ..taxonomy import FilePath, LineNumber, Score, GovernanceReport, LintResultList
from ..contract import (
    ILinterAdapterPort,
    ISemanticTracerPort,
    LintPipelineOrchestratorAggregate,
    ServiceContainerAggregate,
)

logger = logging.getLogger("auto_linter.agent")


class LintPipelineOrchestrator(LintPipelineOrchestratorAggregate):
    """Agent orchestrator that coordinates the linting pipeline.

    This is the "User" or "Orchestrator" that knows how to use different
    Capabilities and Infrastructure to achieve the final goal.
    """

    @property
    def _INTERFACE_PORT(self) -> Type[ILinterAdapterPort]:
        return ILinterAdapterPort

    @property
    def _INTERFACE_TRACER(self) -> Type[ISemanticTracerPort]:
        return ISemanticTracerPort

    def __init__(self, container: ServiceContainerAggregate):
        super().__init__(container=container)

    @property
    def adapters(self) -> list[ILinterAdapterPort]:
        return self.container.adapters

    @property
    def tracers(self) -> dict[str, ISemanticTracerPort]:
        return self.container.tracers

    @property
    def threshold(self) -> Score:
        return self.container.config.thresholds.score

    async def run(self, path: FilePath) -> GovernanceReport:
        """Runs the full pipeline: Scan -> Enrich -> Evaluate."""
        # Accept str for surface convenience; convert to FilePath internally
        if isinstance(path, str):
            path = FilePath(value=path)
        report = GovernanceReport()

        # Step 1: Scanning (Infrastructure)
        async def scan_adapter(adapter):
            try:
                return await adapter.scan(path)
            except Exception as e:
                logger.error(f"Error in adapter {adapter.name()}: {e}")
                return []

        import asyncio
        import os
        from pathlib import Path

        target_path_str = os.path.abspath(str(path))
        is_dir = os.path.isdir(target_path_str)

        all_results = await asyncio.gather(*(scan_adapter(a) for a in self.adapters))
        for results in all_results:
            if isinstance(results, LintResultList):
                for res in results:
                    res_file_str = os.path.abspath(str(res.file))
                    if is_dir:
                        try:
                            Path(res_file_str).relative_to(Path(target_path_str))
                            report.add_result(res)
                        except ValueError:
                            pass
                    else:
                        if res_file_str == target_path_str:
                            report.add_result(res)
            else:
                logger.error(f"Scan failed or adapter failed: {results}")

        # Step 2: Enrichment (Infrastructure/Capability)
        self._enrich_results(report, path)

        # Step 3: Evaluation (Domain Logic)
        report.update_compliance(self.threshold)

        return report

    def _enrich_results(self, report: GovernanceReport, root_path: FilePath):
        """Enriches lint results with semantic context (tracers)."""

        for res in report.results:
            file_path_str = str(res.file)
            tracer = (
                self.tracers.get("python")
                if file_path_str.endswith(".py")
                else self.tracers.get("js")
            )
            if not tracer:
                continue

            # Enrichment logic (formerly in RunAnalysisUseCase)
            if hasattr(tracer, "get_enclosing_scope"):
                scope = tracer.get_enclosing_scope(
                    FilePath(value=file_path_str), LineNumber(value=int(res.line))
                )
                if scope:
                    res.enclosing_scope = scope
